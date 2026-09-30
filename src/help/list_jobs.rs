use {
    crate::*,
    std::io::{
        self,
        Write,
    },
    termimad::{
        MadSkin,
        minimad::{
            OwningTemplateExpander,
            TextTemplate,
        },
    },
};

pub fn write_jobs(
    w: &mut impl Write,
    settings: &Settings,
) -> io::Result<()> {
    static MD: &str = r"
    |:-:|:-|
    |**job**|**command**|
    |:-:|:-|
    ${jobs
    |${job_name}|${job_command}|
    }
    |-|-|
    default job: ${default_job}
    ";
    let mut expander = OwningTemplateExpander::new();
    let mut jobs: Vec<_> = settings.jobs.iter().collect();
    jobs.sort_by_key(|(name, _)| (*name).clone());
    for (name, job) in &jobs {
        expander
            .sub("jobs")
            .set("job_name", name)
            .set("job_command", job.command.join(" "));
    }
    expander.set("default_job", &settings.default_job);
    let skin = MadSkin::default();
    skin.write_owning_expander(w, &expander, &TextTemplate::from(MD))
}
