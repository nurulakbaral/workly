import { Welcome } from "~/src/ui"
import type { Route } from "./+types/home"

export function meta({ data }: Route.MetaArgs) {
  return [{ title: "New React Router App" }, { name: "description", content: "Welcome to React Router!" }]
}

export default function Home() {
  return <Welcome name="Nufik" />
}
