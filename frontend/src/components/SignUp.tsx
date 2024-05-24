import React from "react";
import { getAuth, signInWithPopup, GoogleAuthProvider } from "firebase/auth";
import { app } from "../lib/firebaseConfig";
import { userFromFirebaseUser } from "../lib/user";

const SignUp: React.FC = () => {
  const handleSignIn = () => {
    const auth = getAuth(app);
    const provider = new GoogleAuthProvider();
    signInWithPopup(auth, provider)
      .then((result) => {
        // The signed-in user info.
        const user = result.user;
        const userDraft = userFromFirebaseUser(user);
        console.log(userDraft);
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
