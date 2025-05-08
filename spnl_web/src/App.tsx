import { Page, PageSection } from "@patternfly/react-core"

import Masthead from "./Masthead"
import Body from "./Body"
import "@patternfly/react-core/dist/styles/base.css"

function App() {
  return (
    <Page masthead={<Masthead/>}       >
      <PageSection>
      <Body/>
      </PageSection>
    </Page>
  )
}

export default App
