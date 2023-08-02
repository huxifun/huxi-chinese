# coding=utf-8
import string

def readfile(file_name):
    f = open(file_name,'r')
    src = f.read().strip()
    f.close()
    return src

def writefile(file_name, str):
    f = open(file_name,'w')
    f.write(str)
    f.close()
    return True

old = readfile('huxi.txt')
head = ''';fcitx4 Huxi 码表文件
键码='/;abcdefghijklmnopqrstuvwxyz
码长=3
规避字符=
拼音=@
拼音长度=12
[数据]
'''
all = old.split('\n')
l = []
n = 0
for line in all:
    n = n + 1
    print(str(n) + '----' + line)
    w = line.split(' ')
    data = w[0] + ' ' + w[1]
    l.append(data)
    if len(w) >= 3:
        for next in w[2:]:
            data = w[0] + ' ' + next
            l.append(data)

new = head + '\n'.join(l)
writefile('fcitx.txt', new)

print('run command:')
print('    txt2mb fcitx.txt ~/.config/fcitx/table/huxi.mb')
print('    cp huxi.conf ~/.config/fcitx/table/')

