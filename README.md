# Mig
本ソフトウェアMigはmgファイルからLaravelやRailsのマイグレーションファイルを生成するソフトウェアです。

mgファイルの使用は次のようになります。ただし各\<\>と\<\>の間の区切りはスペースを表すこととします。\_は半角スペース。

\<mg_file\> = \<command\> \<table_name\> { \<table_body\> }<br/>
\<command\> = :create<br/>
\<table_name\> = \<ascii_string\><br/>
\<ascii_string\> = \<ascii_char\>+<br/>
\<ascii_chars\> = [a-zA-Z]<br/>
\<space\> = (\t | \_)\* <br/>
\<space_newline\> = (\<space\> | \n)+<br/>
\<table_body\> = { (\<column\>\<space_newline\>)\* }<br/>
\<column\> = \<column_name\> { \<opt_name\> (\<opt\> \<space\>)\* }<br/>
\<column_name\> = \<ascii_string\><br/>
\<opt_name\> = :\<ascii_string\><br/>
\<opt> = \<String\> | \<Integer\> | \<Double\> | \<Y-m-d\> | \<Time\> | \<Date_Time\> <br/>
\<String\> = " .\*  "<br/>
\<Sign\> = \+ | -<br/>
\<Integer\> = \<Sign\>(0 | [1-9][0-9]\*)<br/>
\<Double\> = \<Sign\>(0\.[0-9]\* |[1-9][0-9]\*\.[0-9]\*)<br/>
\<Y-m-d\> ="(0 | [1-9][0-9]\*)-[1-12]-[1-31]"<br/>
\<Date_Time\> = "\<Y-m-d\>\_\<Time\>"<br/>

# 使い方
最低限指定した場合のコマンドは次の通りです。

```mig.exe -I <input-file> -O <output-file> --target <target-FW>```

