import axios from "axios"
import { useEffect } from "react"
import { api } from "../api/api"

export const Users = () => {

  useEffect(() => {
    (async () => {
      const { data } = await api.get("/users")
      console.log(data)
    })()
  }, [])

  return (
    <div>
      <h1>Users page</h1>
    </div>
  )
}
