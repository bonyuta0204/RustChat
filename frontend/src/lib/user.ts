import { User as FirebaseUser } from "firebase/auth";

export type User = {
  id: string;
  uid: string;
  email: string;
  name: string;
  provider_id: string;
  created_at: string;
  updated_at: string;
};

/**
 * UserDraft is a type that represents a user draft.
 * It is used to create a new user.
 */
export type UserDraft = Omit<User, "id" | "created_at" | "updated_at">;

export function userFromFirebaseUser(user: FirebaseUser): UserDraft {
  const providerData = user.providerData[0];
  if (!providerData) {
    throw new Error("User has no provider data");
  }
  if (!providerData.email || !providerData.displayName) {
    throw new Error("Failed to get user data from firebase");
  }
  return {
    uid: user.uid,
    email: providerData.email,
    name: providerData.displayName,
    provider_id: providerData.providerId,
  };
}
