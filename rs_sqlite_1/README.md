# rs_sqlite_1

 Version: 0.9.1

 date    : 2026/06/10
 
 update :

***

Rust Windows , C++ call TODO SQLite

* rustc 1.93.0 
* Visual Studio Community 2026
* LLVM CLang
* make
* windows11

***
### related

https://www.sqlite.org/download.html

* sqlite-amalgamation-*.zip , download
* sqlite3.h , sqlite3.c

***
* lib add

```
.\vcpkg install nlohmann-json:x64-windows
```
***
* build

```
make all
cargo build
```
***
* add
```
target\debug\rs_sqlite_1.exe  add test1
```

* list
```
target\debug\rs_sqlite_1.exe list
```

* delete
```
target\debug\rs_sqlite_1.exe del 1
```

***


