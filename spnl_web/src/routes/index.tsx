import { createFileRoute } from "@tanstack/react-router"
import Body, { type BodyProps } from "../Body.tsx"

export const Route = createFileRoute("/")({
  component: Index,
  validateSearch: (search: Record<string, unknown>): BodyProps => ({
    qv: search.qv === true || search.qv === "true",
  }),
})

function Index() {
  const props = Route.useSearch()
  return <Body {...props} />
}
