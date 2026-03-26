// SPDX-License-Identifier: PMPL-1.0-or-later

//! The `init` command: initialize a new draft from a Nickel template.

use intsoc_nickel::NickelWorkspace;
use std::path::Path;

/// Run the init command.
pub async fn run(
    name: &str,
    stream: &str,
    group: Option<&str>,
    dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate draft name format
    if !name.starts_with("draft-") {
        return Err(format!("Draft name must start with 'draft-', got '{name}'").into());
    }

    tracing::info!("Initializing draft: {name} (stream: {stream})");

    // Determine the template to use
    let template_type = match stream {
        "individual" => "individual-draft",
        "wg" => {
            if group.is_none() {
                return Err("Working group stream requires --group <wg-abbrev>".into());
            }
            "wg-draft"
        }
        "irtf" => {
            if group.is_none() {
                return Err("IRTF stream requires --group <rg-abbrev>".into());
            }
            "rg-draft"
        }
        "iab" => "iab-document",
        "independent" => "independent-submission",
        _ => return Err(format!("Unknown stream type: {stream}").into()),
    };

    // Try to find Nickel workspace
    let nickel_dir = dir.join("nickel");
    if nickel_dir.exists() {
        let workspace = NickelWorkspace::new(&nickel_dir);
        let template = workspace.template_for_stream(
            match stream {
                "individual" | "wg" => "ietf",
                "irtf" => "irtf",
                "iab" => "iab",
                "independent" => "independent",
                _ => "ietf",
            },
            template_type,
        );

        if template.exists() {
            println!("Using Nickel template: {}", template.display());
            match intsoc_nickel::render::render_template(&template) {
                Ok(output) => {
                    let output_file = dir.join(format!("{name}.xml"));
                    std::fs::write(&output_file, &output)?;
                    println!("Created: {}", output_file.display());
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!("Nickel rendering failed, using built-in template: {e}");
                }
            }
        }
    }

    // Fall back to built-in XML template
    let xml = generate_xml_template(name, stream, group);
    let output_file = dir.join(format!("{name}.xml"));
    std::fs::write(&output_file, &xml)?;
    println!("Created: {}", output_file.display());

    Ok(())
}

fn generate_xml_template(name: &str, _stream: &str, group: Option<&str>) -> String {
    let workgroup = group.map_or(String::new(), |g| {
        format!("    <workgroup>{g}</workgroup>\n")
    });

    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE rfc [
  <!ENTITY nbsp "&#160;">
  <!ENTITY zwsp "&#8203;">
  <!ENTITY nbhy "&#8209;">
  <!ENTITY wj "&#8288;">
]>
<rfc
  xmlns:xi="http://www.w3.org/2001/XInclude"
  category="info"
  docName="{name}"
  ipr="trust200902"
  submissionType="IETF"
  consensus="true"
  version="3">

  <front>
    <title>TODO: Document Title</title>
    <seriesInfo name="Internet-Draft" value="{name}"/>
{workgroup}
    <author fullname="TODO: Author Name" surname="TODO">
      <organization>TODO: Organization</organization>
      <address>
        <email>TODO: email@example.com</email>
      </address>
    </author>

    <date year="2026"/>

    <area>General</area>

    <abstract>
      <t>TODO: Write abstract.</t>
    </abstract>
  </front>

  <middle>
    <section title="Introduction">
      <t>TODO: Write introduction.</t>
    </section>

    <section title="Security Considerations">
      <t>TODO: Describe security considerations.</t>
    </section>

    <section title="IANA Considerations">
      <t>This document has no IANA actions.</t>
    </section>
  </middle>

  <back>
    <references title="Normative References">
    </references>

    <references title="Informative References">
    </references>
  </back>
</rfc>
"#
    )
}
