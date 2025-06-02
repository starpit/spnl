import { createFileRoute } from "@tanstack/react-router"
import Body, { type BodyProps } from "../../Body.tsx"

export const Route = createFileRoute("/demos/$demo")({
  component: Demo,
  validateSearch: (
    search: Record<string, unknown>,
  ): Omit<BodyProps, "demo"> => ({
    qv: !search.qv ? undefined : search.qv === true || search.qv === "true",
    model: search.model,
  }),
})

function Demo() {
  const { demo } = Route.useParams()
  const props = Route.useSearch()
  return <Body {...props} demo={demo} />
}
