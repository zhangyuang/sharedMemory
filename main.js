const koa = require('koa')
const { fork } = require('child_process')

const sharedMemory = require('./index')

const stringLink = "string.link"

sharedMemory.init()
sharedMemory.setString("string.link", "parent")
console.log('Read shared string in parent process:', sharedMemory.getString(stringLink))
const child = fork('./child')

child.send('ready')
child.on('message', msg => {
  if (msg === 'finish') {
    console.log('Read new string in parent process:', sharedMemory.getString(stringLink))
    sharedMemory.clear(stringLink)
  }
})
// // napi.setFunc(func)
// // napi.getFunc(func)

// napi.setObject({foo: 'bar'})
// const bar = napi.getObject()
// console.log(bar)