use std::fs;

use zed_extension_api::{self as zed, LanguageServerId, Result, settings::LspSettings};

struct AmberLsp {
    path: String,
    args: Option<Vec<String>>,
}

struct AmberExtension {
    cached_binary_path: Option<String>,
}

impl AmberExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<AmberLsp> {
        let binary_settings = LspSettings::for_worktree("amber-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());
        
        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(AmberLsp {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(AmberLsp {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "amber-lang/amber-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();

        let asset_name = format!(
            "amber-lsp-{arch}-{os}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 | zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name.starts_with(&asset_name))
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("amer-lsp-{}", release.version);
        let binary_path = format!("{version_dir}/{asset_name}/amber-lsp");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(AmberLsp {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for AmberExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let amber_lsp_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: amber_lsp_binary.path,
            args: amber_lsp_binary
                .args
                .unwrap_or_else(|| vec![]),
            env: Default::default(),
        })
    }
}

zed::register_extension!(AmberExtension);
