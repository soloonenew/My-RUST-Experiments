
// AntiCensorship Tool
// Copyright (C) 2018-2020 M.Anish <aneesh25861@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

/* This is a UnStable Release of AntiCensorship Tool in RUST . Developed by M.Anish only */

use std::io;
use std::process;
use std::process::Command;

/* This Function reads input and returns string */
fn input()->String{
let mut str=String::new();
io::stdin().read_line(&mut str).expect("Input Error!");
str
}//end of input()

/* This Function returns user's browser preference */
fn get_browser()->String
{
let browser;
println!("\n\nSelect Your Browser:-");
println!("\n1)Firefox\n2)Chromium\n3)Chrome\n\nEnter choice:");
let ch=input();
let ch : i32 = ch.trim().parse().expect("Invalid Choice!");
if ch==1{
browser="firefox";
}
else if ch==2{
browser="chromium";
}
else if ch==3{
browser="chrome";
}
else
{
process::exit(1);
} browser.to_string()
}//end of get_browser()

/* Function to load proxified website in browser */
fn process(browser : String, url : String , service : String )
{

let mut cmd: String = service.to_owned();
    let borrowed_string: String = url;
    cmd.push_str(&borrowed_string);

Command::new(browser)
            .arg(cmd)
            .output()
            .expect("failed to execute process");
}//end of process()

/* Function to load given website in browser */
fn web_process(browser : String, url : String)
{

Command::new(browser)
            .arg(url)
            .output()
            .expect("failed to execute process");
}// end of web_browser()

fn main()
{
let browser=get_browser();
println!("\n\nEnter website URL:");
let url=input();

println!("\n\nSelect Services To Unblock Website:-\n\n");
println!("Difficult To Block Services\n\n 1)Archive Fo \n 2)Internet Archive \n 3)Google Cache \n 4)Googleweblight \n 5)Searx \n 6)Startpage \n 7)Hypothes.is \n 8)Webpage to pdf using pdfcrowd \n 9)Webpage to pdf using pdfmyurl \n\nProxy Sites \n\n 10)Proxysite \n 11)Hidester Proxy \n 12)Kproxy  \n 13)Hide.me Proxy\n 14)HMA Proxy \n\nEnter Choice:");
let x=input();
let x : i32 = x.trim().parse().expect("Error");
if x==1{
process(browser,url,"https://archive.fo/?run=1&url=".to_string());   
}
else if x==2{
process(browser,url,"https://web.archive.org/save/_embed/".to_string());
}
else if x==3{
process(browser,url,"https://webcache.googleusercontent.com/search?q=cache:".to_string());
}
else if x==4{
process(browser,url,"https://googleweblight.com/i?u=".to_string());
}
else if x==5{
process(browser,url,"https://searx.info/?q=".to_string());
}
else if x==6{
process(browser,url,"https://www.startpage.com/do/search?&q=".to_string());
}
else if x==7{
process(browser,url,"https://via.hypothes.is/".to_string());
}
else if x==8{
web_process(browser,"https://pdfcrowd.com/".to_string());
}
else if x==9{
web_process(browser,"https://pdfmyurl.com/".to_string());
}
else if x==10{
web_process(browser,"https://proxysite.com".to_string());
}
else if x==11{
web_process(browser,"https://hidester.com/proxy/".to_string());
}
else if x==12{
web_process(browser,"https://kproxy.com/".to_string());
}
else if x==13{
web_process(browser,"https://hide.me/en/proxy".to_string());
}
else if x==14{
web_process(browser,"https://www.hidemyass.com/proxy".to_string());
}
else
{
process::exit(1);
}
}

