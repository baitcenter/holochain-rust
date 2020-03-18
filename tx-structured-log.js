#!/usr/bin/env node

function get_json_lines(cb) {
  return new Promise((resolve, reject) => {
    let chunks = []

    const checkProcess = () => {
      chunks = [chunks.join('')]
      let idx = chunks[0].indexOf('\n')
      while (idx > -1) {
        const line = chunks[0].substr(0, idx)
        chunks[0] = chunks[0].substr(idx + 1)

        const m = line.trim().match(/(?:^({.*)|<SL<(.*)>SL>)/)
        if (m && m.length == 3) {
          let data
          try {
            data = JSON.parse(m[1] || m[2])
          } catch (e) {
            continue
          }
          cb(data)
        }

        idx = chunks[0].indexOf('\n')
      }
    }

    process.stdin.setEncoding('utf8')

    process.stdin.on('readable', () => {
      while ((chunk = process.stdin.read()) !== null) {
        chunks.push(chunk)
        checkProcess()
      }
    })

    process.stdin.on('end', () => {
      chunks.push('\n')
      checkProcess()
      resolve()
    })

    process.stdin.on('error', e => {
      reject(e)
    })
  })
}

async function main() {
  let first_time = null
  const req_ids = {}
  const open_dms = {}
  await get_json_lines(data => {
    const timestamp = Date.parse(data.time)
    let t = 0
    if (!first_time) {
      first_time = timestamp
    } else {
      t = timestamp - first_time
    }
    t = (t / 1000).toFixed(3)
    let since_req_origin = null
    if (data.fields.request_id) {
      if (data.fields.request_id in req_ids) {
        since_req_origin = timestamp - req_ids[data.fields.request_id]
      } else {
        since_req_origin = 0
        req_ids[data.fields.request_id] = timestamp
      }
    }
    /*
    if (data.msg_type === 'HandleSendDirectMessage') {
      open_dms[data.request_id] = true
    } else if (data.msg_type === 'HandleSendDirectMessageResult') {
      delete open_dms[data.request_id]
    }
    */
    since_req_origin = (since_req_origin / 1000).toFixed(3)
    console.log('fu')
    console.log(JSON.stringify({
      time: data.time,
      time_diff: t,
      since_req_origin,
      time_since_last: data.fields.time_since_last,
      level: data.level,
      tag: data.fields.tag,
      dir: data.fields.dir,
      msg_type: data.fields.msg_type,
      uri: data.fields.uri,
      request_id: data.fields.request_id,
      entry_address: data.fields.entry_address,
      from_agent_id: data.fields.from_agent_id,
      to_agent_id: data.fields.to_agent_id,
      data: data.fields.data || data.fields.message,
      file: data.file,
      line: data.line,
      module_path: data.module_path,
    }))
  })
  //console.error('UNFINISHED DirectMessage count', Object.keys(open_dms).length)
}

// entry point
main().then(() => {}, err => {
  console.error(err)
  process.exit(1)
})