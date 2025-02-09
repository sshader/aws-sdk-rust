// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateAgentKnowledgeBase`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_id(impl Into<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::agent_id) / [`set_agent_id(Option<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::set_agent_id):<br>required: **true**<br>Id generated at the server side when an Agent is created<br>
    ///   - [`agent_version(impl Into<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::agent_version) / [`set_agent_version(Option<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::set_agent_version):<br>required: **true**<br>Draft Version of the Agent.<br>
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::set_knowledge_base_id):<br>required: **true**<br>Id generated at the server side when a Knowledge Base is associated to an Agent<br>
    /// - On success, responds with [`DisassociateAgentKnowledgeBaseOutput`](crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseOutput)
    /// - On failure, responds with [`SdkError<DisassociateAgentKnowledgeBaseError>`](crate::operation::disassociate_agent_knowledge_base::DisassociateAgentKnowledgeBaseError)
    pub fn disassociate_agent_knowledge_base(
        &self,
    ) -> crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder {
        crate::operation::disassociate_agent_knowledge_base::builders::DisassociateAgentKnowledgeBaseFluentBuilder::new(self.handle.clone())
    }
}
