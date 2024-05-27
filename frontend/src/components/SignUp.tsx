import React from "react";
import { getAuth, signInWithPopup, GoogleAuthProvider } from "firebase/auth";
import { app } from "../lib/firebaseConfig";
import { userFromFirebaseUser } from "../lib/user";
import apiClient from "../lib/api_client";

const SignUp: React.FC = () => {
  const handleSignIn = () => {
    const auth = getAuth(app);
    const provider = new GoogleAuthProvider();
    signInWithPopup(auth, provider)
      .then(async (result) => {
        // The signed-in user info.
        const user = result.user;
        const userDraft = userFromFirebaseUser(user);

        const response = await apiClient.post("/users", userDraft);
        console.log(response);
      })
      .catch((error) => {
        console.log(error);
      });
  };

  return (
    <div>
      <h1>Sign In</h1>
      <button onClick={handleSignIn}>Sign In with Google</button>
    </div>
  );
};

export default SignUp;
