import { createFileRoute } from "@tanstack/react-router"
import Body, { type BodyProps } from "../../Body.tsx"

export const Route = createFileRoute("/demos/$demo")({
  component: Demo,
  validateSearch: (
    search: Record<string, unknown>,
  ): Omit<BodyProps, "demo"> => ({
    qv: search.qv === true || search.qv === "true",
  }),
})

function Demo() {
  const { demo } = Route.useParams()
  const props = Route.useSearch()
  return <Body {...props} demo={demo} />
}
