#[derive(Debug, Clone)]
pub enum Exam {
    Ssc,
    Jsc,
    SscVoc,
    Hsc,
    HscVoc,
    HscHbm,
    HscDic,
}

impl Exam {
    pub fn as_str(&self) -> &'static str {
        match self {
            Exam::Ssc => "ssc",
            Exam::Jsc => "jsc",
            Exam::SscVoc => "ssc_voc",
            Exam::Hsc => "hsc",
            Exam::HscVoc => "hsc_voc",
            Exam::HscHbm => "hsc_hbm",
            Exam::HscDic => "hsc_dic",
        }
    }
}
