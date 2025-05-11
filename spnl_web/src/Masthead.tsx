import {
  Masthead,
  MastheadMain,
  MastheadBrand,
  MastheadLogo,
  MastheadContent,
} from "@patternfly/react-core"

import ModelDownloader from "./ModelDownloader"

export default function SPNLMasthead() {
  return (
    <Masthead>
      <MastheadMain>
        <MastheadBrand>
          <MastheadLogo>Span Query Playground</MastheadLogo>
        </MastheadBrand>
      </MastheadMain>
      <MastheadContent>
        <ModelDownloader />
      </MastheadContent>
    </Masthead>
  )
}
