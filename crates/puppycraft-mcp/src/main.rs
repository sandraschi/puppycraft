use clap::{Parser, Subcommand};
use puppycraft_core::command::{dispatch_tool, tools_list};
use puppycraft_core::skills::{export_skills, skill_by_uri, skill_read, skills_list};
use puppycraft_core::store::PuppyStore;
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "puppycraft-mcp", about = "PuppyCraft CLI & MCP stdio server")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run MCP 2024-11-05 JSON-RPC 2.0 stdio server
    Mcp,
    /// Manage bundled skills
    Skills {
        #[command(subcommand)]
        sub: SkillsCmd,
    },
    /// Show PuppyCraft status
    Status {
        #[arg(long)]
        json: bool,
    },
    /// Match dog breeds for a household preset (oldie, family_kids_cat, almoehi, city_single, sporty)
    Match {
        #[arg(long, default_value = "city_single")]
        preset: String,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    /// Look up a dog breed
    Breed { query: String },
    /// Look up travel rules for a city or country (e.g. Wien, Tokio)
    Travel { destination: String },
    /// Check toxic substance
    Toxic { substance: String },
}

#[derive(Subcommand)]
enum SkillsCmd {
    List {
        #[arg(long)]
        json: bool,
    },
    Show {
        name: String,
    },
    Export {
        #[arg(long)]
        to: PathBuf,
        #[arg(long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("puppycraft-mcp error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let store = PuppyStore::open_default().map_err(|e| e.to_string())?;
    match cli.cmd {
        Cmd::Mcp => run_mcp_stdio(&store),
        Cmd::Skills { sub } => match sub {
            SkillsCmd::List { json: as_json } => {
                let items = skills_list();
                if as_json {
                    let v: Vec<Value> = items
                        .iter()
                        .map(|s| json!({ "name": s.name, "description": s.description, "uri": format!("skill://{}/SKILL.md", s.name) }))
                        .collect();
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
                    );
                } else {
                    for s in items {
                        println!("{} - {}", s.name, s.description);
                    }
                }
                Ok(())
            }
            SkillsCmd::Show { name } => {
                let s = skill_read(&name).map_err(|e| e.to_string())?;
                println!("{}", s.skill_md);
                Ok(())
            }
            SkillsCmd::Export { to, force } => {
                let paths = export_skills(&to, force).map_err(|e| e.to_string())?;
                for p in paths {
                    println!("{}", p.display());
                }
                Ok(())
            }
        },
        Cmd::Status { json: _ } => {
            let v = dispatch_tool(&store, "puppycraft_status", &json!({}))
                .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Cmd::Match { preset, limit } => {
            let v = dispatch_tool(
                &store,
                "puppycraft_match_breeds",
                &json!({ "preset": preset, "limit": limit }),
            )
            .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Cmd::Breed { query } => {
            let v = dispatch_tool(&store, "puppycraft_breed_info", &json!({ "query": query }))
                .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Cmd::Travel { destination } => {
            let v = dispatch_tool(
                &store,
                "puppycraft_travel_guide",
                &json!({ "destination": destination }),
            )
            .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Cmd::Toxic { substance } => {
            let v = dispatch_tool(
                &store,
                "puppycraft_toxic_check",
                &json!({ "substance": substance }),
            )
            .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?
            );
            Ok(())
        }
    }
}

fn handle_jsonrpc(store: &PuppyStore, req: &Value) -> Option<Value> {
    let id = req.get("id")?.clone();
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or_else(|| json!({}));

    let resp = match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": { "listChanged": false },
                    "resources": { "listChanged": false }
                },
                "serverInfo": {
                    "name": "puppycraft-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        }),
        "tools/list" => {
            let tools: Vec<Value> = tools_list()
                .into_iter()
                .map(|t| {
                    json!({
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": t.input_schema
                    })
                })
                .collect();
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "tools": tools }
            })
        }
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            match dispatch_tool(store, name, &args) {
                Ok(val) => {
                    let txt = serde_json::to_string_pretty(&val).unwrap_or_default();
                    json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": txt }],
                            "isError": false
                        }
                    })
                }
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{ "type": "text", "text": format!("{}: {}", e.kind(), e) }],
                        "isError": true
                    }
                }),
            }
        }
        "resources/list" => {
            let res: Vec<Value> = skills_list()
                .iter()
                .map(|s| {
                    json!({
                        "uri": format!("skill://{}/SKILL.md", s.name),
                        "name": s.name,
                        "description": s.description,
                        "mimeType": "text/markdown"
                    })
                })
                .collect();
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "resources": res }
            })
        }
        "resources/read" => {
            let uri = params.get("uri").and_then(Value::as_str).unwrap_or("");
            match skill_by_uri(uri) {
                Ok(s) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "contents": [{
                            "uri": uri,
                            "mimeType": "text/markdown",
                            "text": s.skill_md
                        }]
                    }
                }),
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32002,
                        "message": e.to_string()
                    }
                }),
            }
        }
        "ping" => json!({ "jsonrpc": "2.0", "id": id, "result": {} }),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": format!("Method not found: {method}") }
        }),
    };
    Some(resp)
}

fn run_mcp_stdio(store: &PuppyStore) -> Result<(), String> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                let err = json!({
                    "jsonrpc": "2.0",
                    "id": Value::Null,
                    "error": { "code": -32700, "message": format!("Parse error: {e}") }
                });
                writeln!(stdout, "{err}").map_err(|ioe| ioe.to_string())?;
                stdout.flush().map_err(|ioe| ioe.to_string())?;
                continue;
            }
        };
        if let Some(resp) = handle_jsonrpc(store, &req) {
            writeln!(stdout, "{resp}").map_err(|e| e.to_string())?;
            stdout.flush().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_resources_and_tools_work() -> Result<(), Box<dyn std::error::Error>> {
        let store = PuppyStore::open_in_memory()?;
        let init = handle_jsonrpc(
            &store,
            &json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }),
        )
        .ok_or("missing init reply")?;
        assert!(init["result"]["capabilities"]["resources"].is_object());

        let rlist = handle_jsonrpc(
            &store,
            &json!({ "jsonrpc": "2.0", "id": 2, "method": "resources/list" }),
        )
        .ok_or("missing resources/list")?;
        let arr = rlist["result"]["resources"].as_array().ok_or("arr")?;
        assert_eq!(arr.len(), 2);

        let rread = handle_jsonrpc(
            &store,
            &json!({ "jsonrpc": "2.0", "id": 3, "method": "resources/read", "params": { "uri": "skill://puppycraft-advisor/SKILL.md" } }),
        )
        .ok_or("missing resources/read")?;
        assert!(rread["result"]["contents"][0]["text"]
            .as_str()
            .unwrap_or("")
            .contains("PuppyCraft Dog Advisor Skill"));

        let missing = handle_jsonrpc(
            &store,
            &json!({ "jsonrpc": "2.0", "id": 4, "method": "resources/read", "params": { "uri": "skill://nope/SKILL.md" } }),
        )
        .ok_or("missing error")?;
        assert_eq!(missing["error"]["code"], -32002);
        Ok(())
    }
}
