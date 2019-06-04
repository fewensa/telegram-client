use crate::types::t_user_type_bot::TGUserTypeBot;

impl TGUserTypeBot {
  /// True, if the bot can be invited to basic group and supergroup chats.
  pub fn can_join_groups(&self) -> bool { self.origin().can_join_groups().map_or(false, |v| v) }

  /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages.
  pub fn can_read_all_group_messages(&self) -> bool { self.origin().can_read_all_group_messages().map_or(false, |v| v) }

  /// True, if the bot supports inline queries.
  pub fn is_inline(&self) -> bool { self.origin().is_inline().map_or(false, |v| v) }

  /// Placeholder for inline queries (displayed on the client input field).
  pub fn inline_query_placeholder(&self) -> Option<String> { self.origin().inline_query_placeholder().clone() }

  /// True, if the location of the user should be sent with every inline query to this bot.
  pub fn need_location(&self) -> bool { self.origin().need_location().map_or(false, |v| v) }
}
