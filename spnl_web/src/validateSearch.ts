import { type SearchSchemaInput } from "@tanstack/react-router"
import { type BodyProps } from "./Body.tsx"

export default function validateSearch(
  search: Omit<BodyProps, "demo"> & SearchSchemaInput,
): Omit<BodyProps, "demo"> {
  return {
    qv: !search.qv ? undefined : search.qv === true || search.qv === "true",
    model: typeof search.model === "string" ? search.model : undefined,
  }
}
