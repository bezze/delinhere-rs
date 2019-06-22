" Initialize the channel
if !exists('s:appJobId')
	let s:appJobId = 0
endif

" Initialize RPC
function! s:initRpc()
  if s:appJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:appJobId
  endif
endfunction

" Constants for RPC messages.
let s:Add = 'add'

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:bin = '/home/teseo/Other/gits/delinhere-rs/target/debug/delinhere-rs'

function! s:configureCommands()
  command! -nargs=+ Add :call s:add(<f-args>)
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
  " call rpcnotify(id, s:Add, 'asd 123 asd')
  endif
endfunction

function! s:add(...)
  let s:p = get(a:, 1, 0)
  let s:q = get(a:, 2, 0)

  echo "Calling Add"
  call rpcnotify(s:appJobId, s:Add, a:)
endfunction

call s:connect()
