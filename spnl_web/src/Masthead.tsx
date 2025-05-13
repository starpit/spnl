import {
  Masthead,
  MastheadMain,
  MastheadBrand,
  MastheadLogo,
  MastheadContent,
} from "@patternfly/react-core"

import DemoSelect from "./DemoSelect"
import ModelSelect from "./ModelSelect"

export default function SPNLMasthead() {
  return (
    <Masthead>
      <MastheadMain>
        <MastheadBrand>
          <MastheadLogo>Span Query Playground</MastheadLogo>
        </MastheadBrand>
      </MastheadMain>
      <MastheadContent>
        <DemoSelect />
        <ModelSelect />
      </MastheadContent>
    </Masthead>
  )
}
