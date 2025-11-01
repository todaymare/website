use std::{fmt::Write, time::SystemTime};

use atom_syndication::Category;
use chrono::Datelike;
use markdown::{Options, ParseOptions};
use rayon::iter::{ParallelBridge, ParallelIterator};
use rss_gen::{generate_rss, RssVersion};
use webp::WebPConfig;

fn main() {
    let index_template = include_str!("../index_template.html");
    let blog_template = include_str!("../blog_template.html");
    let mut blogs = vec![];
    let mut dirs = vec![];

    dirs.push((false, std::fs::read_dir("blogs").unwrap()));
    dirs.push((true, std::fs::read_dir("hidden_blogs").unwrap()));

    for (is_hidden, dir) in dirs {
        for item in dir {
            let item = item.unwrap();
            let name = item.file_name().to_string_lossy().to_string();
            let metadata = item.metadata().unwrap();

            if !metadata.is_dir() {
                println!("skipping '{name}' because it's not a valid directory");
                continue;
            }

            let index = item.path().join("index.md");
            let created = std::fs::metadata(item.path().join("thumbnail.png")).map(|x| x.created().unwrap()).unwrap_or(SystemTime::now());
            let index = std::fs::read_to_string(index).unwrap();

            std::fs::write(&format!("{}/index.html", &*item.path().to_string_lossy()), markdown::to_html(&index)).unwrap();

            blogs.push(Blog {
                ident: item.path().to_string_lossy().to_string(),
                markdown: index,
                creation_date: created,
                is_hidden,
            });

        }
    }

    blogs.sort_by_key(|x| x.creation_date);

    let mut blogs_section = String::new();
    let mut rss = rss_gen::RssData::new(Some(RssVersion::RSS2_0))
        .title("daymare.net")
        .link("https://daymare.net/")
        .description("Explore my personal projects, technical blogs, and creative coding experiments at daymare.net.")
        .language("en-us");

    let mut atom_entries = vec![];

    let mut entries = blogs.iter().rev().enumerate().par_bridge()
    .map(|(index, blog)| {
        let ident = &blog.ident;
        let title = blog.markdown.lines().next().unwrap_or("# Untitled");
        let title = title.split_once('#').unwrap().1.trim();

        let read_time = {
            let word_count = blog.markdown.split_whitespace().count();
            let wpm = 200;
            word_count.div_ceil(wpm)
        };

        let mut description = String::new();

        for line in blog.markdown.lines().skip(1) {
            description.push_str(line);
            if line.ends_with('\\') && !line.ends_with("\\\\") {
                description.pop();
                continue;
            }
            break;
        }

        let description_html = markdown::to_html_with_options(
            &description,
            &Options::default(),
        ).unwrap();


        let thumbnail_file = format!("{}/thumbnail.png", ident);
        let thumbnail = if std::fs::exists(&thumbnail_file).unwrap() { thumbnail_file.as_str() }
                        else { "https://placehold.co/1900x160" };

        if !std::fs::exists(format!("{ident}/assets")).unwrap() {
            std::fs::create_dir(format!("{ident}/assets")).unwrap();
        }

        // downscale thumbnail & convert to webp
        'b: {
            // if we already have generated the thumbnail and the file is newer than the source, skip
            let thumbnail_webp = format!("{}/assets/banner_1600x1347.webp", ident);

            if std::fs::exists(&thumbnail_webp).unwrap() {
                let thumb_meta = std::fs::metadata(&thumbnail_webp).unwrap();
                let source_meta = std::fs::metadata(&thumbnail_file).unwrap();
                if thumb_meta.modified().unwrap() > source_meta.modified().unwrap() {
                    println!("skipping thumbnail generation for {ident}, already exists and is up to date");
                    break 'b;
                }
            } else {
                println!("generating thumbnails for {ident}");
            }

            let img = image::open(&thumbnail_file).unwrap();
            assert!(img.width() == 1900 && img.height() == 1600, "thumbnail image must be 1900x1600 pixels");

            for size in [304, 400, 800] {
                let resized = img.resize_exact(size, size * 1600 / 1900, image::imageops::FilterType::Lanczos3);
                let rgba = resized.to_rgba8();
                let output_path = format!("{}/assets/thumbnail_{}x{}.webp", ident, size, size * 1600 / 1900);

                let encoder = webp::Encoder::from_rgba(&rgba, resized.width(), resized.height());

                let mut config = WebPConfig::new().unwrap();
                config.lossless = 0;
                config.quality = 85.0;
                config.method = 6;
                config.sns_strength = 60;
                config.filter_strength = 30;
                config.filter_sharpness = 4;
                config.autofilter = 1;
                config.alpha_compression = 1;
                config.alpha_quality = 90;
                config.pass = 2;
                config.use_sharp_yuv = 1;
                config.exact = 0;
                
                let data = encoder.encode_advanced(&config).unwrap();
                std::fs::write(&output_path, &*data).unwrap();
            }

            println!("generating banner images for {ident}");

            for size in [50, 400, 800, 1200, 1600] {
                let resized = img.resize_exact(size, size * 1600 / 1900, image::imageops::FilterType::Lanczos3);
                let rgba = resized.to_rgba8();
                let output_path = format!("{}/assets/banner_{}x{}.webp", ident, size, size * 1600 / 1900);

                let encoder = webp::Encoder::from_rgba(&rgba, resized.width(), resized.height());

                let mut config = WebPConfig::new().unwrap();
                config.lossless = 1;
                config.quality = 100.0; // ignored when lossless, but explicit
                config.method = 6;
                config.near_lossless = 90; // use 100 for true lossless
                config.exact = 1;
                config.use_sharp_yuv = 1;
                config.alpha_compression = 1;
                config.alpha_quality = 100;
                
                let data = encoder.encode_advanced(&config).unwrap();
                std::fs::write(&output_path, &*data).unwrap();
            }
            
        }


        // the blog's index.html
        let html = markdown::to_html(blog.markdown.split_once('\n').unwrap().1);
        let date = chrono::DateTime::<chrono::prelude::Utc>::from(blog.creation_date);
        let month = match date.month() {
            1  => "Jan",
            2  => "Feb",
            3  => "Mar",
            4  => "Apr",
            5  => "May",
            6  => "Jun",
            7  => "Jul",
            8  => "Aug",
            9  => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => unreachable!(),

        };


        let base64_thumbnail = {
            let thumb_img = std::fs::read(&format!("{}/assets/banner_50x42.webp", ident)).unwrap();
            base64::encode(&thumb_img)
        };


        let iso_date = date.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
        let template = blog_template
            .replace("<!-- expand-date -->", &format!("{} {}", month, date.day()))
            .replace("<!-- expand-iso-date -->", &iso_date)
            .replace("<!-- expand-title -->", &title)
            .replace("<!-- expand-read-time -->", &read_time.to_string())
            .replace("<!-- expand-description -->", &description)
            .replace("<!-- expand-path -->", &ident)
            .replace("<!-- expand-body -->", &html)
            .replace("<!-- expand-blurbase64 -->", &base64_thumbnail);


        // replace ~~any~~ with strikethrough
        let template = {
            let mut output = String::new();
            let mut chars = template.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '~' && chars.peek() == Some(&'~') {
                    // skip the next '~'
                    chars.next();
                    output.push_str("<del>");
                    while let Some(nc) = chars.next() {
                        if nc == '~' && chars.peek() == Some(&'~') {
                            // skip the next '~'
                            chars.next();
                            output.push_str("</del>");
                            break;
                        } else {
                            output.push(nc);
                        }
                    }
                } else {
                    output.push(c);
                }
            }
            output
        };


        // replace <img src="%.webm"> with <video> tag
        let template = {
            let mut output = String::new();
            let mut chars = template.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '<' && chars.peek() == Some(&'i') {
                    let mut tag = String::new();
                    tag.push(c);
                    while let Some(nc) = chars.next() {
                        tag.push(nc);
                        if nc == '>' {
                            break;
                        }
                    }
                    if tag.contains("<img") && tag.contains("src=\"") && tag.contains(".webm\"") && tag.contains("alt=\"auto\"") {
                        // extract src
                        let src_start = tag.find("src=\"").unwrap() + 5;
                        let src_end = tag[src_start..].find('"').unwrap() + src_start;
                        let src = &tag[src_start..src_end];
                        output.push_str(&format!(
                            "<video autoplay loop muted playsinline data-src=\"{src}\" type=\"video/webm\"></video>",
                        ));
                    } else if !tag.contains("kofi_symbol.svg") && false {
                        tag.insert_str(5, "loading=\"lazy\" ");
                        output.push_str(&tag);
                    } else {
                        output.push_str(&tag);
                    }
                } else {
                    output.push(c);
                }
            }
            output
        };

        // replace all aref links to open a new tab except the home class link
        let template = {
            let mut output = String::new();
            let mut chars = template.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '<' && chars.peek() == Some(&'a') {
                    let mut tag = String::new();
                    tag.push(c);
                    while let Some(nc) = chars.next() {
                        tag.push(nc);
                        if nc == '>' {
                            break;
                        }
                    }
                    if tag.contains("<a") && tag.contains("href=\"") && !tag.contains("class=\"home\"") {
                        // insert target="_blank" rel="noopener noreferrer"
                        let insert_pos = tag.find('>').unwrap();
                        let (start, end) = tag.split_at(insert_pos);
                        let new_tag = format!(
                            "{} target=\"_blank\" rel=\"noopener noreferrer\"{}",
                            start, end
                        );
                        output.push_str(&new_tag);
                    } else {
                        output.push_str(&tag);
                    }
                } else {
                    output.push(c);
                }
            }
            output
        };

        std::fs::write(format!("{ident}/index.html"), template).unwrap();

        // generate rss item
        if blog.is_hidden {
            return None;
        }

        let blogs_section = format!(
            "
                <a class=\"blog-card\" href=\"{ident}\">
                    <!-- <img src=\"{thumbnail}\" alt=\"Blog Image\"> -->
                    <img src=\"{ident}/assets/thumbnail_800x673.webp\" alt=\"Blog Image\" srcset=\"
                        {ident}/assets/thumbnail_304x256.webp 304w,
                        {ident}/assets/thumbnail_400x336.webp 400w,
                        {ident}/assets/thumbnail_800x673.webp 800w,
                    \" sizes=\"19rem\">
                    <span class=\"titlecard\"><h3>{title}</h3></span>
                    <h4>{read_time} min. read</h4>
                    <p>{description_html}</p>
                </a>
            "
        );


        let rfc_date = date.format("%a, %d %b %Y %H:%M:%S %z").to_string();
        let rss_item = rss_gen::RssItem::new()
            .title(title)
            .link(format!("https://daymare.net/{}", ident))
            .description(description)
            .guid(format!("https://daymare.net/{}", ident))
            .pub_date(rfc_date)
            .enclosure(format!("https://daymare.net/{}/thumbnail.png", ident));


        let atom_item = atom_syndication::Entry {
            title: title.into(),
            id: format!("https://daymare.net/{}", ident),
            updated: date.into(),
            authors: vec![atom_syndication::Person {
                name: "daymare".into(),
                email: None,
                uri: Some("https://daymare.net/".into()),
            }],
            links: vec![atom_syndication::Link {
                href: format!("https://daymare.net/{}", ident),
                rel: "alternate".into(),
                mime_type: None,
                hreflang: None,
                title: None,
                length: None,
            }],
            content: Some(atom_syndication::Content {
                value: Some(html.replace("<img src=\"", format!("<img src=\"https://daymare.net/{}/", ident).as_str())),
                src: None,
                content_type: Some("html".into()),
                ..Default::default()
            }),
            categories: vec![Category {
                term: "Blog".into(),
                scheme: None,
                label: None,
            }],
            published: Some(date.into()),
            summary: Some(description_html.into()),
            ..Default::default()
        };

        Some((index, blogs_section, rss_item, atom_item))
    })
    .filter_map(|value| value)
    .collect::<Vec<_>>();

    entries.sort_by_key(|entry| entry.0);

    for entry in entries {
        blogs_section.push_str(&entry.1);
        rss.add_item(entry.2);
        atom_entries.push(entry.3);
    }

    let atom = atom_syndication::Feed {
        title: "daymare.net".into(),
        id: "https://daymare.net/".into(),
        updated: chrono::Utc::now().into(),
        authors: vec![atom_syndication::Person {
            name: "daymare".into(),
            email: None,
            uri: Some("https://daymare.net/".into()),
        }],
        links: vec![atom_syndication::Link {
            href: "https://daymare.net/".into(),
            rel: "self".into(),
            mime_type: None,
            hreflang: None,
            title: None,
            length: None,
        }],
        entries: atom_entries,
        ..Default::default()
    };


    let output = index_template.replace("<!-- expand-blogs -->", &blogs_section);
    std::fs::write("index.html", output).unwrap();
    std::fs::write("rss.xml", generate_rss(&rss).unwrap()).unwrap();
    std::fs::write("atom.xml", atom_syndication::Feed::to_string(&atom)).unwrap();

}


struct Blog {
    ident: String,
    markdown: String,
    creation_date: SystemTime,
    is_hidden: bool,
}
