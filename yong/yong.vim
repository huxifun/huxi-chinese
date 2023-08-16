" yong input method
let g:yong_vim = "/home/cy/.emacs.d/yong/l64/yong-vim"
let g:input_toggle = 0
let g:input_toggle_timer = 0

fun! Yong_set_input_toggle(channel,output)
    let g:input_toggle = a:output
endf

fun! Yong2en_timer(timer)
    if g:input_toggle_timer == 0
        return
    endif
    let g:input_toggle_timer = 0
    call job_start(g:yong_vim . " 1 -w",{'out_cb':'Yong_set_input_toggle'})
endf

fun! Yong2en()
    let g:input_toggle_timer=1
    call timer_start(200,'Yong2en_timer')
endf

fun! Yong2zh()
    let g:input_toggle_timer=0
    if g:input_toggle != 0
        call job_start(g:yong_vim . " 0")
        g:input_toggle = 0
    endif
endf

autocmd InsertLeave * call Yong2en()
autocmd InsertEnter * call Yong2zh() 
