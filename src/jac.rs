use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct JacExtension;

impl zed::Extension for JacExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let env = binary_settings
            .as_ref()
            .and_then(|settings| settings.env.clone())
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();

        let args = binary_settings
            .as_ref()
            .and_then(|settings| settings.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_string()]);

        if let Some(path) = binary_settings.and_then(|settings| settings.path) {
            return Ok(zed::Command {
                command: path,
                args,
                env,
            });
        }

        match worktree.which("jac") {
            Some(path) => Ok(zed::Command {
                command: path,
                args,
                env,
            }),
            None => Err(
                "Unable to find `jac` on PATH. Install it with `pip install jaclang`.".into(),
            ),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|settings| settings.initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|settings| settings.settings)
    }
}

zed::register_extension!(JacExtension);
