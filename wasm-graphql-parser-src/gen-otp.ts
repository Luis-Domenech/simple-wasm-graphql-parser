import { authenticator } from 'otplib'


export const gen_otp = async () => {
  if (!process.env.NPM_2FA_SECRET) {
    console.error("NPM_2FA_SECRET is not set in environment")
    process.exit(1)
  }
  
  const otp = await authenticator.generate(process.env.NPM_2FA_SECRET!)
  return otp
}
