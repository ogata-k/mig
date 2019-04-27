# Mig
本ソフトウェアMigはmgファイルからLaravelやRailsのマイグレーションファイルを生成するソフトウェアです。

mgファイルの使用は次のようになります。ただし各<>と<>の間の区切りはスペースを表すこととします。\_は半角スペース。
<mg_file> = <command> <table_name> { <table_body> }
<command> = :create
<table_name> = <ascii_string>
<ascii_string> = <ascii_char>+
<ascii_chars> = [a-zA-Z]
<space> = (\t | \_)* 
<space_newline> = (<space> | \n)+
<table_body> = { (<column><space_newline>)* }
<column> = <column_name> { <opt_name> (<opt> <space>)* }
<column_name> = <ascii_string>
<opt_name> = :<ascii_string>
<opt> = <String> | <Integer> | <Double> | <Y-m-d> | <Time> | <Date_Time> 
<String> = " .*  "
<Sign> = \+ | -
<Integer> = <Sign>(0 | [1-9][0-9]*)
<Double> = <Sign>(0\.[0-9]* |[1-9][0-9]*\.[0-9]*)
<Y-m-d> ="(0 | [1-9][0-9]*)-[1-12]-[1-31]"
<Date_Time> = "<Y-m-d>\_<Time>"

# 使い方
最低限指定した場合のコマンドは次の通りです。
```mig -i <target_file> --type=<target_FW> -o <output_file>```

