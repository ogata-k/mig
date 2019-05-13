use crate::app::converter::mig::Mig;

pub trait Generator {
    fn gen_header(&self, mig: &Mig, name_space: String) -> String;
    fn gen_column_options(&self, mig: &Mig) -> String;
    fn gen_table_options(&self, mig: &Mig) -> String;
    fn gen_up(&self, mig: &Mig) -> String;
    fn gen_down(&self, mig: &Mig) -> String;
    fn gen_footer(&self, mig: &Mig) -> String;
    fn generate(&self, mig: &Mig, name_space: String) -> String {
        return format!("{}{}\n{}{}\n",
                       self.gen_header(mig, name_space),
                       self.gen_up(mig),
                       self.gen_down(mig),
                       self.gen_footer(mig)
        );
    }
}