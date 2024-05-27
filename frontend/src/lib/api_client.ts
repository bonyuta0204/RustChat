import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from "axios";

class APIClient {
  private axiosInstance: AxiosInstance;

  constructor(baseURL: string) {
    this.axiosInstance = axios.create({
      baseURL: baseURL,
      headers: {
        "Content-Type": "application/json",
      },
    });
  }

  public async get<T>(url: string): Promise<T> {
    const response = await this.axiosInstance.get<T>(url);
    return response.data;
  }

  public async post<T>(
    url: string,
    data?: any,
    config?: AxiosRequestConfig
  ): Promise<T> {
    try {
      const response: AxiosResponse<T> = await this.axiosInstance.post(
        url,
        data,
        config
      );
      return response.data;
    } catch (error) {
      throw error;
    }
  }
}

const apiClient = new APIClient("/api");

export default apiClient;
