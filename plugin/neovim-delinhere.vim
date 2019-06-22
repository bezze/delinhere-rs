" Initialize the channel
if !exists('s:appJobId')
	let s:appJobId = 0
endif

" Initialize RPC
function! s:initRpc()
  if s:appJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return str2nr(jobid)
  else
    return str2nr(s:appJobId)
  endif
endfunction

" Constants for RPC messages.
let s:DelInHere = 'DelInHere'
let s:DelArHere = 'DelArHere'
let s:ChaInHere = 'ChaInHere'
let s:ChaArHere = 'ChaArHere'
let s:SelInHere = 'SelInHere'
let s:SelArHere = 'SelArHere'
let s:YanInHere = 'YanInHere'
let s:YanArHere = 'YanArHere'
let s:Test = 'Test'

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
" let s:bin = '/home/jason/Others/gits/delinhere-rs/target/debug/delinhere-rs'
let s:bin = '/home/teseo/Other/gits/delinhere-rs/target/debug/delinhere-rs'

function! s:configureCommands()
    " command! -nargs=+ Add :call s:add(<f-args>)
    command! -nargs=0 DIHTest :call s:test(<f-args>)
    command! -nargs=0 DIHDelInHere :call s:delinhere(<f-args>)
    command! -nargs=0 DIHDelArHere :call s:delarhere(<f-args>)
    command! -nargs=0 DIHChaInHere :call s:chainhere(<f-args>)
    command! -nargs=0 DIHChaArHere :call s:chaarhere(<f-args>)
    command! -nargs=0 DIHSelInHere :call s:selinhere(<f-args>)
    command! -nargs=0 DIHSelArHere :call s:selarhere(<f-args>)
    command! -nargs=0 DIHYanInHere :call s:yaninhere(<f-args>)
    command! -nargs=0 DIHYanArHere :call s:yanarhere(<f-args>)

    nnoremap dih  :DIHDelInHere<CR>
    nnoremap dah  :DIHDelArHere<CR>
    nnoremap cih  :DIHChaInHere<CR>
    nnoremap cah  :DIHChaArHere<CR>
    nnoremap vih  :DIHSelInHere<CR>
    nnoremap vah  :DIHSelArHere<CR>
    nnoremap yih  :DIHYanInHere<CR>
    nnoremap yah  :DIHYanArHere<CR>
    nnoremap ;t  :DIHTest<CR>
    nnoremap ;T  :call Testmatch()<CR>

endfunction

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()

  if 0 == id
    echoerr "delinhere-rs: cannot start rpc process"
  elseif -1 == id
    echoerr "delinhere-rs: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:appJobId = id

    call s:configureCommands()
  endif
endfunction

function! Testmatch()
    let back=searchpairpos('{','','}','bnW', '(synIDattr(synID(line("."), col("."), 0), "name") =~? "string\\|comment")')
    let forw=searchpairpos('{','','}','nW', '(synIDattr(synID(line("."), col("."), 0), "name") =~? "string\\|comment")')
    echo string(back) . string(forw)
endfunction

function! s:test(...)
  call rpcnotify(s:appJobId, s:Test, a:)
endfunction

function! s:delinhere(...)
  call rpcnotify(s:appJobId, s:DelInHere, a:)
endfunction

function! s:delarhere(...)
  call rpcnotify(s:appJobId, s:DelArHere, a:)
endfunction

function! s:chainhere(...)
  call rpcnotify(s:appJobId, s:ChaInHere, a:)
endfunction

function! s:chaarhere(...)
  call rpcnotify(s:appJobId, s:ChaArHere, a:)
endfunction

function! s:selinhere(...)
  call rpcnotify(s:appJobId, s:SelInHere, a:)
endfunction

function! s:selarhere(...)
  call rpcnotify(s:appJobId, s:SelArHere, a:)
endfunction

function! s:yaninhere(...)
  call rpcnotify(s:appJobId, s:YanInHere, a:)
endfunction

function! s:yanarhere(...)
  call rpcnotify(s:appJobId, s:YanArHere, a:)
endfunction

call s:connect()
