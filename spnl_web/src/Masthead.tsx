import {
  Flex,
  Masthead,
  MastheadMain,
  MastheadToggle,
  MastheadBrand,
  MastheadLogo,
  MastheadContent,
  PageToggleButton,
  Title,
} from "@patternfly/react-core"

export default function SPNLMasthead() {
  return (
    <Masthead>
      <MastheadMain>
        <MastheadBrand>
          <MastheadLogo>Span Query Playground</MastheadLogo>
        </MastheadBrand>
      </MastheadMain>
      <MastheadContent></MastheadContent>
    </Masthead>
  )
}
