//    This file is part of StultusVisio.
//
//    StultusVisio is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    StultusVisio is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>6.
//    Jefferson T. @ 2023. Telegram: StalinCCCP
use std::fs;
use base64;

pub enum Handle {
    Caption,
    Image,
    Video,
    URLVideo,
    Text,
    Heading,
    SubHeading,
    List,
    OrdList,
    Mermaid,
}

pub fn trim_element(input: &String) -> String {
    if let Some(index) = input.find(' ') {
        let cut_string = input[index + 1..].to_string();
        cut_string
    } else {
        input.to_string() //isso deve ser um erro!
    }
}

pub fn close_last_handle(handle: &Option<Handle>) -> &str {
    match handle {
        None => "",
        Some(Handle::Image) => "</figure>",
        Some(Handle::Mermaid) => "</pre></div>",
        Some(Handle::Caption) => "</figure>",
        Some(Handle::List) => "</ul>",
        Some(Handle::OrdList) => "</ol>",
        Some(Handle::Text) => "</p>",
        Some(Handle::Heading) => "</h1>",
        Some(Handle::SubHeading) => "</h2>",
        Some(Handle::Video) => "</video>",
        Some(Handle::URLVideo) => "</iframe></div>",
    }
}

pub fn file_base64(file: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_data = fs::read(file)?;
    Ok(base64::encode(&file_data))
}

pub fn generate_mermaid_script(mermaid_script: Option<String>) -> String {
    match mermaid_script {
        Some(mermaid_script) => format!(
                "<script type=\"module\">import mermaid from '{}';</script>", 
                mermaid_script),
        None => "<script type=\"module\">import mermaid from 
            'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
            </script>".to_string()
    }
}

pub fn generate_logo(logo_path: Option<String>) -> String {
    match logo_path {
        Some(logo_path) => format!("<img src=\"{}\" class=\"logo\">", logo_path),
        None => "".to_string(),
    }
}
