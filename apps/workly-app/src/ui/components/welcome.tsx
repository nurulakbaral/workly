import * as React from "react"

type TWelcomeProps = {
  name: string
}

export function Welcome({ name, ...props }: TWelcomeProps) {
  const [count, setCount] = React.useState(0)

  return (
    <div {...props}>
      <h1 className="text-center">Welcome: {name}</h1>
      <p className="text-center">Count: {count}</p>
      <button onClick={() => setCount(count + 1)} type="button">
        Increment
      </button>
    </div>
  )
}
