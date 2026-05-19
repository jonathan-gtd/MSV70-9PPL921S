use std::path::Path;
use quick_xml::{events::{BytesEnd, BytesStart, BytesText, Event}, Reader, Writer};
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct CustomJob {
    pub ecu:        String,
    pub jobname:    String,
    pub virtualname: String,
    pub comment:    String,
    pub pids:       Vec<u16>,
}

impl CustomJob {
    pub fn argument_string(&self) -> String {
        let hex_pids: Vec<String> = self.pids.iter().map(|p| format!("0x{:04X}", p)).collect();
        format!("3;{}", hex_pids.join(";"))
    }

    pub fn pid_count(&self) -> usize {
        self.pids.len()
    }
}

pub fn load_customjobs(path: &Path) -> Result<Vec<CustomJob>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read customjobs.xml: {e}"))?;

    let mut reader = Reader::from_str(&content);
    reader.config_mut().trim_text(true);

    let mut jobs: Vec<CustomJob> = Vec::new();
    let mut cur: Option<CustomJob> = None;
    let mut cur_elem = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = std::str::from_utf8(e.name().as_ref()).unwrap_or("").to_string();
                cur_elem = name.clone();
                if name == "customjob" {
                    let mut job = CustomJob {
                        ecu: String::new(), jobname: String::new(), virtualname: String::new(),
                        comment: String::new(), pids: Vec::new(),
                    };
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"ecu" {
                            job.ecu = String::from_utf8_lossy(&attr.value).to_string();
                        }
                    }
                    cur = Some(job);
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if let Some(ref mut job) = cur {
                    match cur_elem.as_str() {
                        "jobname"    => job.jobname = text,
                        "virtualname"=> job.virtualname = text,
                        "comment"    => job.comment = text,
                        "argument"   => {
                            job.pids = parse_argument(&text);
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"customjob" {
                    if let Some(job) = cur.take() {
                        jobs.push(job);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {e}")),
            _ => {}
        }
    }

    Ok(jobs)
}

pub fn save_customjobs(path: &Path, jobs: &[CustomJob]) -> Result<(), String> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None)))
        .map_err(|e| e.to_string())?;

    writer.write_event(Event::Start(BytesStart::new("customjobs")))
        .map_err(|e| e.to_string())?;

    for job in jobs {
        let mut start = BytesStart::new("customjob");
        start.push_attribute(("ecu", job.ecu.as_str()));
        writer.write_event(Event::Start(start)).map_err(|e| e.to_string())?;

        write_elem(&mut writer, "jobname", &job.jobname)?;
        write_elem(&mut writer, "virtualname", &job.virtualname)?;
        write_elem(&mut writer, "comment", &job.comment)?;
        write_elem(&mut writer, "argument", &job.argument_string())?;

        writer.write_event(Event::End(BytesEnd::new("customjob"))).map_err(|e| e.to_string())?;
    }

    writer.write_event(Event::End(BytesEnd::new("customjobs"))).map_err(|e| e.to_string())?;

    let xml_bytes = writer.into_inner().into_inner();
    std::fs::write(path, xml_bytes).map_err(|e| format!("Cannot write: {e}"))?;
    Ok(())
}

fn write_elem(writer: &mut Writer<Cursor<Vec<u8>>>, tag: &str, text: &str) -> Result<(), String> {
    writer.write_event(Event::Start(BytesStart::new(tag))).map_err(|e| e.to_string())?;
    writer.write_event(Event::Text(BytesText::new(text))).map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new(tag))).map_err(|e| e.to_string())?;
    Ok(())
}

fn parse_argument(s: &str) -> Vec<u16> {
    // Format: "3;0x4807;0x5804;..."
    s.split(';')
        .skip(1)  // skip the leading "3"
        .filter_map(|p| {
            u16::from_str_radix(p.trim().trim_start_matches("0x"), 16).ok()
        })
        .collect()
}
