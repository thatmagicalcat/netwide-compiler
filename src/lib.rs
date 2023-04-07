use wandbox::{CompilationBuilder, Wandbox};

#[derive(Debug, Clone)]
pub struct Output {
    pub program_stdout: String,
    pub program_stderr: String,

    pub compiler_stdout: String,
    pub compiler_stderr: String,
}

pub struct NetWideCompiler {
    languages: Vec<(String, Vec<String>)>,
    wbox: Wandbox,
}

impl NetWideCompiler {
    pub async fn new() -> Self {
        let wbox = Wandbox::new(None, None).await.unwrap();

        Self {
            languages: wbox
                .get_languages()
                .into_iter()
                .map(|x| {
                    (
                        x.name,
                        x.compilers
                            .into_iter()
                            .map(|compiler| compiler.name)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect(),
            wbox,
        }
    }

    pub fn check(&self, lang: &str, target: Option<&str>) -> Result<(), String> {
        let Some(lang_match) = self.languages
            .iter()
            .find(|l| l.0 == lang)
            else { return Err("Language not found".to_string()); };

        if target.is_none() { return Ok(()); }

        matches!(
            lang_match.1.iter().find(|t| t.as_str() == target.unwrap()),
            None
        )
        .then(|| Err("Target not found".to_string()))
        .unwrap_or(Ok(()))
    }

    /// Target is chosen automatically when None is given
    pub async fn run(
        &self,
        lang: &str,
        target: Option<&str>,
        code: String,
    ) -> Result<Output, String> {
        if let Err(e) = self.check(lang, target) {
            return Err(e);
        }

        let mut builder = CompilationBuilder::new();

        builder.code(&code);

        builder.target(if let Some(target) = target {
            target
        } else {
            self.get_compilers(lang).unwrap().first().unwrap().as_str()
        });

        if let Err(e) = builder.build(&self.wbox) {
            return Err(e.to_string());
        }

        let res = match builder.dispatch().await {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let program_stdout = res.program_stdout;
        let program_stderr = res.program_stderr;
        let compiler_stdout = res.compiler_stdout;
        let compiler_stderr = res.compiler_stderr;

        Ok(Output {
            program_stdout,
            program_stderr,
            compiler_stdout,
            compiler_stderr,
        })
    }

    // pub fn print_langs(&self) {
    //     println!("langs: {:#?}", self.languages);
    // }

    fn get_compilers(&self, lang: &str) -> Option<&Vec<String>> {
        let Some(lang_match) = self.languages
            .iter()
            .find(|l| l.0 == lang)
        else { return None; };

        Some(&lang_match.1)
    }
}
