const { spawn, exec } = require('child_process')

const REPS = 5

const run_bench = (command, args, dir) => {
  return new Promise((resolve, reject) => {
    const serve = spawn(command, args, { cwd: dir })

    serve.stdout.on('data', data => {
      console.log('Serve Data:', data.toString())

      if (data.toString().split(' ').includes('listening')) {
        const bench = spawn('wrk', ['-t6', '-c512', '-d30s', 'http://localhost:3000'])

        bench.stdout.on('data', data => console.log('Benchmark data: ', data.toString()))
        bench.stderr.on('data', data => reject(data.toString()))

        bench.on('close', code => {
          console.log('Benchmark complete')
          serve.kill()
        })
      }
    })

    serve.stderr.on('data', data => reject(data.toString()))

    serve.on('exit', code => {
      console.log(`Serve Closed`)
      resolve()
    })
  })
}

const benchmark = async (command, args, dir) => {
  for (let i = 0; i < REPS; i++) {
    try {
      await run_bench(command, args, dir)
    } catch (e) {
      console.error(e)
    }
  }
}

// benchmark('yarn', ['serve'], './ts-server')
benchmark('cargo', ['run', '--release'], './rocket_server')
