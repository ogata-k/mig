use crate::app::converter::mig::Mig;
use crate::app::converter::generator::Generator;
use crate::app::helper::string_helper::to_head_large;

pub struct Laravel {}

// TODO 変換時の仕様を決めること
impl Generator for Laravel {
    fn gen_header(&self, mig: &Mig, name_space: String) -> String {
        let imports: String = format!("<?php\n\n\
            namespace {};\n\n\
            use Illuminate\\Support\\Facades\\Schema;\n\
            use Illuminate\\Database\\Schema\\Blueprint;\n\
            use Illuminate\\Database\\Migrations\\Migration;\n\n"
                                      , name_space);
        let class_name: String = [
            mig.method.to_lowercase(),
            mig.table_name.to_lowercase(),
            "Table".to_string()
        ].iter().map(|s| to_head_large(s)).collect::<Vec<String>>().join("");
        let class_def: String = format!("class {} extends Migration\n{{\n", class_name);

        return format!("{}{}", imports, class_def);
    }

    fn gen_column_options(&self, mig: &Mig) -> String {
        return "hogehoge_column\n".to_string();
        unimplemented!()
    }

    fn gen_table_options(&self, mig: &Mig) -> String {
        return "fugfuga_table\n".to_string();
        unimplemented!()
    }

    fn gen_up(&self, mig: &Mig) -> String {
        let param_comment = "\t/**\n\t* マイグレーション実行\n\t*\n\t* @return void\n\t*/\n";
        let func_name = format!(
            "\tpublic function up()\n\t{{\n\t\tSchema::create('{}', function (Blueprint $table) {{\n",
            mig.table_name.to_lowercase()
        );

        return format!("{}{}{}{}\t\t}});\n\t}}\n", param_comment, func_name, self.gen_column_options(mig), self.gen_table_options(mig));
    }

    fn gen_down(&self, mig: &Mig) -> String {
        let param_comment = "\t/**\n\t* マイグレーションを元に戻す\n\t*\n\t* @return void\n\t*/\n";
        let func = format!(
            "\tpublic function down()\n\t{{\n\t\tSchema::drop('{}');\n\t}}\n",
            mig.table_name.to_lowercase()
        );
        return format!("{}{}", param_comment, func);
    }

    fn gen_footer(&self, mig: &Mig) -> String {
        return "}\n".to_string();
    }
}