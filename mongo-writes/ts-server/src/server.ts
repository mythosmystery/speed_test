import express from 'express'
import { MongoClient, Db } from 'mongodb'
import dotenv from 'dotenv'
dotenv.config()

const app = express()

const client = new MongoClient(process.env.MONGO_URI || 'mongodb://localhost:27017/speed_test')
const COLLECTION_NAME = 'test_collection'

let db: Db

const connect = async () => {
  try {
    const conn = await client.connect()
    db = conn.db('speed_test')
    await client.db('admin').command({ ping: 1 })
    await db.dropCollection(COLLECTION_NAME)
    await db.createCollection(COLLECTION_NAME)
    console.log('Connected to MongoDB')
  } catch (error) {
    console.error(error)
  }
}

app.get('/', async (_, res) => {
  try {
    await db.collection(COLLECTION_NAME).insertOne({
      name: 'Test Name',
      description: 'Test Description',
      random_number: Math.random(),
      created_at: new Date()
    })
    res.sendStatus(200)
  } catch (error) {
    console.error(error)
    res.sendStatus(500)
  }
})

app.listen(3000, async () => {
  await connect()
  console.log('Example app listening on port 3000!')
})
