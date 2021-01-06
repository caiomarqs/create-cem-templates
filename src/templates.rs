
pub fn create_html_str(day: &String) -> String {

    let title = format!("<title>Day - {}</title>", day);

    let index_str_pt1 = String::from(r#"
<!DOCTYPE html>    
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="style.css">
    "#);
    
    let index_str_pt2 = String::from(r#"
</head>
<body>
   
</body>
</html>   
    "#);
    
    let index_str = format!(r#"
{}
    {}
{}   
"#, index_str_pt1, title, index_str_pt2);

    index_str
}


pub fn create_style_str() -> String {
    let css = String::from(r#"
html {
    box-sizing: border-box;
}
        
*, *:before, *:after {
    box-sizing: inherit;
}
      
body {
    margin: 0;
    padding: 0;
    font-weight: normal;
}
    "#);

    css
}