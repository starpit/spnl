import {
  Masthead,
  MastheadMain,
  MastheadBrand,
  MastheadLogo,
  MastheadContent,
} from "@patternfly/react-core"

import DemoSelect from "./DemoSelect"
import ModelSelect from "./ModelSelect"

type Props = {
  demo: string
}

export default function SPNLMasthead(props: Props) {
  return (
    <Masthead>
      <MastheadMain>
        <MastheadBrand>
          <MastheadLogo>Span Query Playground</MastheadLogo>
        </MastheadBrand>
      </MastheadMain>
      <MastheadContent>
        <DemoSelect demo={props.demo} />
        <ModelSelect />
      </MastheadContent>
    </Masthead>
  )
}
