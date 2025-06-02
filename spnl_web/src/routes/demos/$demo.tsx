import { createFileRoute } from "@tanstack/react-router"

import Body from "../../Body.tsx"
import validateSearch from "../../validateSearch.ts"

export const Route = createFileRoute("/demos/$demo")({
  component: Demo,
  validateSearch,
})

function Demo() {
  const { demo } = Route.useParams()
  const props = Route.useSearch()
  return <Body {...props} demo={demo} />
}
