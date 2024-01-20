use aho_corasick::{AhoCorasick, MatchKind};
use azusa::Azusa;
use napi::{bindgen_prelude::AsyncTask, Env, Error, Result, Task};
use once_cell::sync::OnceCell;

static GLOBAL_PATTERNS: OnceCell<Vec<String>> = OnceCell::new();

#[napi(object)]
#[derive(Debug, Default)]
pub struct Input {
    pub haystack: String,
    pub patterns: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Debug, Default)]
pub struct FindOutput {
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub pattern: Option<i64>,
    pub matched: bool,
}

#[napi(object)]
pub struct Options {
    pub case_insensitive: Option<bool>,
}

pub struct FindTask {
    pub input: Input,
    pub opts: Option<Options>,
}

impl FindTask {
    fn get_pattern(&self) -> Result<&Vec<String>> {
        if let Some(p) = &self.input.patterns {
            Ok(p)
        } else if let Some(p) = GLOBAL_PATTERNS.get() {
            Ok(p)
        } else {
            Err(Error::from_reason(
                "patterns is required, please set it in input or load it by loadPatterns",
            ))
        }
    }

    fn find(&self, mode: MatchKind) -> Result<FindOutput> {
        let mut builder = &mut AhoCorasick::builder();
        if let Some(o) = &self.opts &&
            let Some(v) = o.case_insensitive && v {
            builder = builder.ascii_case_insensitive(true);
        }
        let ac = builder.match_kind(mode).build(self.get_pattern()?).unwrap();
        let haystack = &self.input.haystack;
        let mat = ac.find(haystack.as_str());
        if mat.is_none() {
            Ok(FindOutput {
                matched: false,
                ..Default::default()
            })
        } else {
            let info = mat.unwrap();
            let transformer = Azusa::new(haystack.to_string());
            let js_range = transformer.utf8_to_utf16((info.start() as u32, info.end() as u32));
            Ok(FindOutput {
                matched: true,
                start: Some(js_range.0 as i64),
                end: Some(js_range.1 as i64),
                pattern: Some(info.pattern().as_u64() as i64),
            })
        }
    }

    fn find_all(&self) -> Result<Vec<FindOutput>> {
        let mut builder = &mut AhoCorasick::builder();
        if let Some(o) = &self.opts &&
            let Some(v) = o.case_insensitive && v {
            builder = builder.ascii_case_insensitive(true);
        }
        let ac = builder.build(self.get_pattern()?).unwrap();
        let mut matches = vec![];
        let haystack = &self.input.haystack;
        let transformer = Azusa::new(haystack.to_string());
        for mat in ac.find_iter(haystack.as_str()) {
            let js_range = transformer.utf8_to_utf16((mat.start() as u32, mat.end() as u32));
            matches.push(FindOutput {
                matched: true,
                start: Some(js_range.0 as i64),
                end: Some(js_range.1 as i64),
                pattern: Some(mat.pattern().as_u64() as i64),
            });
        }
        Ok(matches)
    }
}

#[napi]
pub fn find_left_first_match_sync(input: Input, opts: Option<Options>) -> Result<FindOutput> {
    FindTask { input, opts }.find(MatchKind::LeftmostFirst)
}

#[napi]
pub fn find_match_sync(input: Input, opts: Option<Options>) -> Result<FindOutput> {
    FindTask { input, opts }.find(MatchKind::Standard)
}

#[napi]
pub fn find_left_first_longest_match_sync(
    input: Input,
    opts: Option<Options>,
) -> Result<FindOutput> {
    FindTask { input, opts }.find(MatchKind::LeftmostLongest)
}

#[napi]
pub fn find_all_match_sync(input: Input, opts: Option<Options>) -> Result<Vec<FindOutput>> {
    FindTask { input, opts }.find_all()
}

#[napi]
pub fn load_patterns(patterns: Vec<String>) -> Result<()> {
    GLOBAL_PATTERNS.set(patterns).unwrap();
    Ok(())
}

// find all match
pub struct FindAllMatchTask {
    task: FindTask,
}

impl Task for FindAllMatchTask {
    type Output = Vec<FindOutput>;
    type JsValue = Vec<FindOutput>;

    fn compute(&mut self) -> Result<Self::Output> {
        Ok(self.task.find_all()?)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi(ts_return_type = "Promise<FindOutput[]>")]
pub fn find_all_match(input: Input, opts: Option<Options>) -> AsyncTask<FindAllMatchTask> {
    AsyncTask::new(FindAllMatchTask {
        task: FindTask { input, opts },
    })
}

// find match by mode
pub struct FindMatchByModeTask {
    task: FindTask,
    mode: MatchKind,
}

impl Task for FindMatchByModeTask {
    type Output = FindOutput;
    type JsValue = FindOutput;

    fn compute(&mut self) -> Result<Self::Output> {
        Ok(self.task.find(self.mode)?)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi(ts_return_type = "Promise<FindOutput>")]
pub fn find_left_first_longest_match(
    input: Input,
    opts: Option<Options>,
) -> AsyncTask<FindMatchByModeTask> {
    AsyncTask::new(FindMatchByModeTask {
        task: FindTask { input, opts },
        mode: MatchKind::LeftmostLongest,
    })
}

#[napi(ts_return_type = "Promise<FindOutput>")]
pub fn find_match(input: Input, opts: Option<Options>) -> AsyncTask<FindMatchByModeTask> {
    AsyncTask::new(FindMatchByModeTask {
        task: FindTask { input, opts },
        mode: MatchKind::Standard,
    })
}

#[napi(ts_return_type = "Promise<FindOutput>")]
pub fn find_left_first_match(
    input: Input,
    opts: Option<Options>,
) -> AsyncTask<FindMatchByModeTask> {
    AsyncTask::new(FindMatchByModeTask {
        task: FindTask { input, opts },
        mode: MatchKind::LeftmostFirst,
    })
}
