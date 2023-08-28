#!/usr/bin/bash
cp emacs/header.txt emacs/cy-3500.txt
cat 3500.txt >> emacs/cy-3500.txt

cp vim/header.txt vim/hx-3500.txt
cat 3500.txt >> vim/hx-3500.txt

cp yong/header.txt yong/temp.txt
cat 3500.txt >> yong/temp.txt

iconv -f utf8 -t gbk yong/temp.txt > yong/huxi.txt
