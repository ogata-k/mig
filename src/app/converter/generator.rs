use crate::app::converter::mig::Mig;

pub trait Generator {
    fn gen_header(&self, mig: &Mig, name_space: String) -> String;
    fn gen_column_options(&self, mig: &Mig) -> String;
    fn gen_table_options(&self, mig: &Mig) -> String;
    fn gen_body(&self, mig: &Mig) -> String {
        return format!("{}{}", self.gen_column_options(mig), self.gen_table_options(mig));
    }
    fn gen_footer(&self, mig: &Mig) -> String;
    fn generate(&self, mig: &Mig, name_space: String) -> String {
        return format!("{}{}{}\n",
                       self.gen_header(mig, name_space),
                       self.gen_body(mig),
                       self.gen_footer(mig)
        );
    }
}