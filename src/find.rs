use aho_corasick::{AhoCorasick, MatchKind};
use napi::{bindgen_prelude::AsyncTask, Env, Result, Task};

#[napi(object)]
#[derive(Debug, Default)]
pub struct Input {
    pub haystack: String,
    pub patterns: Vec<String>,
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
    fn find(&self, mode: MatchKind) -> FindOutput {
        let mut builder = &mut AhoCorasick::builder();
        if let Some(o) = &self.opts &&
            let Some(v) = o.case_insensitive && v {
            builder = builder.ascii_case_insensitive(true);
        }
        let ac = builder
            .match_kind(mode)
            .build(&self.input.patterns)
            .unwrap();
        let mat = ac.find(&self.input.haystack);
        if mat.is_none() {
            FindOutput {
                matched: false,
                ..Default::default()
            }
        } else {
            let info = mat.unwrap();
            FindOutput {
                matched: true,
                start: Some(info.start() as i64),
                end: Some(info.end() as i64),
                pattern: Some(info.pattern().as_u64() as i64),
            }
        }
    }

    fn find_all(&self) -> Vec<FindOutput> {
        let mut builder = &mut AhoCorasick::builder();
        if let Some(o) = &self.opts &&
            let Some(v) = o.case_insensitive && v {
            builder = builder.ascii_case_insensitive(true);
        }
        let ac = builder.build(&self.input.patterns).unwrap();
        let mut matches = vec![];
        for mat in ac.find_iter(&self.input.haystack) {
            matches.push(FindOutput {
                matched: true,
                start: Some(mat.start() as i64),
                end: Some(mat.end() as i64),
                pattern: Some(mat.pattern().as_u64() as i64),
            });
        }
        matches
    }
}

#[napi]
pub fn find_left_first_match_sync(input: Input, opts: Option<Options>) -> FindOutput {
    FindTask { input, opts }.find(MatchKind::LeftmostFirst)
}

#[napi]
pub fn find_match_sync(input: Input, opts: Option<Options>) -> FindOutput {
    FindTask { input, opts }.find(MatchKind::Standard)
}

#[napi]
pub fn find_left_first_longest_match_sync(input: Input, opts: Option<Options>) -> FindOutput {
    FindTask { input, opts }.find(MatchKind::LeftmostLongest)
}

#[napi]
pub fn find_all_match_sync(input: Input, opts: Option<Options>) -> Vec<FindOutput> {
    FindTask { input, opts }.find_all()
}

// find all match
pub struct FindAllMatchTask {
    task: FindTask,
}

impl Task for FindAllMatchTask {
    type Output = Vec<FindOutput>;
    type JsValue = Vec<FindOutput>;

    fn compute(&mut self) -> Result<Self::Output> {
        Ok(self.task.find_all())
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
        Ok(self.task.find(self.mode))
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
