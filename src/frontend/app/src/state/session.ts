export type SessionState = {
  userId: string | null;
  csrfToken: string | null;
};

export const sessionStateDefault: SessionState = {
  userId: null,
  csrfToken: null,
};
