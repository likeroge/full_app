/* eslint-disable @typescript-eslint/no-explicit-any */
import { useEffect, useState } from "react"
import { api } from "../api/api"

export const Users = () => {

  const [users, setUsers] = useState([])
  useEffect(() => {
    (async () => {
      const { data } = await api.get("/users")
      console.log(data)
      setUsers(data);
    })()
  }, [])

  return (
    <div>
      <h1>Users page</h1>
      <ul>
        {
          users.map((user: any) => <li key={user.id}>{user.name}</li>)
        }
      </ul>
    </div>
  )
}
