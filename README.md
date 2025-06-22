# Dolya

## What is Dolya? <br>
#### Dolya is a simple Google Dorks formatter and runner written in the rust programming language

## Usage: <br>
```
 ____        _             
|  _ \  ___ | |_   _  __ _ 
| | | |/ _ \| | | | |/ _` |
| |_| | (_) | | |_| | (_| |
|____/ \___/|_|\__, |\__,_|
               |___/       

[+] Usage: dolya <search-query> [-options]
    
    options:
        -h, --help                           print the help message
        -p, --print-search-query             will print the full search query instead of searching for it
        -l, --list-browsers                  list all the browsers which exist in the system
        -b, --browser <brwoser-name>         choose the browser you want to use for the search
        -u, --inurl <file>                   search for pages that have spicific word in the url
        -e, --exclude-domains <file>         get the excluded domains for the search from a text file
        -i, --intitle <file>                 search for pages that have spicific word
        -t, --intext <file>                  find pages where the specified word appears in the web page
        -f, --filetype <file>                search for specific file types = filetype:x or filetype:y
        -r, --rules <file>                   use a single file that has all the search operators you want
        -s, --search-engine <search-engine>  choose the search engine you want to use

        Available search engines:            _______________________________________
                                            |   google  |   duckduckgo  |   bing    |
        Made By:                            |___________|_______________|___________|
        ~ MuhammadMuazen ~                  |   yahoo   |     yandex    |   brave   |
        ~ github.com/MuhammadMuazen ~       |___________|_______________|___________|
        
        Note: All the Google Dorks operators use the or method to search
        e.g: operator:<line1> or operator:<line2> where the lines come from the provided files
        
[+] Example(1): dolya "search_query" -b firefox -s brave -u inurl_file.txt -f filetypes.txt
[+] Example(2): dolya "seach_query" -b firefox -s google -r rules_file.txt
```
---
## OS Support?
Right now the project is only compatible with the linux operating system.<br>Support for windows OS will be added later.<br>
## Notes:  (Very Important)
1. Security Alert: this program passes the arguments after formatting them to the command line shell which would open a potential command injection attack. For now please make sure it has only the privileges it needs, no root or admin privileges should be granted.
2. This is my first rust project so there might be a lot of weird approches for some problems but I will optimize the solutions as I am learning.
3. Have fun.
---  
