const koa = require('koa')
const {fork} = require('child_process')

const napi = require('./index')

const func = (a) => {
  console.log(a)
}
napi.createShareMemory()
napi.setFunc(func)

