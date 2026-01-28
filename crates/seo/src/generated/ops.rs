// AUTO-GENERATED. DO NOT EDIT.
#![allow(dead_code, unused_imports)]
use anyhow::Result;
use crate::client::DataForSeoClient;
use crate::generated::models::*;

pub async fn id_list(client: &DataForSeoClient, body: &Vec<SerpIdListRequestInfo>) -> Result<SerpIdListResponseInfo> {
    client.post::<Vec<SerpIdListRequestInfo>, SerpIdListResponseInfo>("/v3/serp/id_list", body).await
}

pub async fn errors(client: &DataForSeoClient, body: &Vec<SerpErrorsRequestInfo>) -> Result<SerpErrorsResponseInfo> {
    client.post::<Vec<SerpErrorsRequestInfo>, SerpErrorsResponseInfo>("/v3/serp/errors", body).await
}

pub async fn screenshot(client: &DataForSeoClient, body: &Vec<SerpScreenshotRequestInfo>) -> Result<SerpScreenshotResponseInfo> {
    client.post::<Vec<SerpScreenshotRequestInfo>, SerpScreenshotResponseInfo>("/v3/serp/screenshot", body).await
}

pub async fn ai_summary(client: &DataForSeoClient, body: &Vec<SerpAiSummaryRequestInfo>) -> Result<SerpAiSummaryResponseInfo> {
    client.post::<Vec<SerpAiSummaryRequestInfo>, SerpAiSummaryResponseInfo>("/v3/serp/ai_summary", body).await
}

pub async fn google_locations(client: &DataForSeoClient) -> Result<SerpGoogleLocationsResponseInfo> {
    let path = "/v3/serp/google/locations".to_string();
    client.get::<SerpGoogleLocationsResponseInfo>(
        &path
    ).await
}

pub async fn google_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpGoogleLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/google/locations/{country}", country = country);
    client.get::<SerpGoogleLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn google_languages(client: &DataForSeoClient) -> Result<SerpGoogleLanguagesResponseInfo> {
    let path = "/v3/serp/google/languages".to_string();
    client.get::<SerpGoogleLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleOrganicTaskPostRequestInfo>) -> Result<SerpGoogleOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleOrganicTaskPostRequestInfo>, SerpGoogleOrganicTaskPostResponseInfo>("/v3/serp/google/organic/task_post", body).await
}

pub async fn google_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/google/organic/tasks_ready".to_string();
    client.get::<SerpGoogleOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn tasks_ready(client: &DataForSeoClient) -> Result<SerpTasksReadyResponseInfo> {
    let path = "/v3/serp/tasks_ready".to_string();
    client.get::<SerpTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/google/organic/tasks_fixed".to_string();
    client.get::<SerpGoogleOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/google/organic/task_get/regular/{id}", id = id);
    client.get::<SerpGoogleOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/organic/task_get/html/{id}", id = id);
    client.get::<SerpGoogleOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_organic_live_regular(client: &DataForSeoClient, body: &Vec<SerpGoogleOrganicLiveRegularRequestInfo>) -> Result<SerpGoogleOrganicLiveRegularResponseInfo> {
    client.post::<Vec<SerpGoogleOrganicLiveRegularRequestInfo>, SerpGoogleOrganicLiveRegularResponseInfo>("/v3/serp/google/organic/live/regular", body).await
}

pub async fn google_organic_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleOrganicLiveAdvancedRequestInfo>) -> Result<SerpGoogleOrganicLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleOrganicLiveAdvancedRequestInfo>, SerpGoogleOrganicLiveAdvancedResponseInfo>("/v3/serp/google/organic/live/advanced", body).await
}

pub async fn google_organic_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleOrganicLiveHtmlRequestInfo>) -> Result<SerpGoogleOrganicLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleOrganicLiveHtmlRequestInfo>, SerpGoogleOrganicLiveHtmlResponseInfo>("/v3/serp/google/organic/live/html", body).await
}

pub async fn google_ai_mode_languages(client: &DataForSeoClient) -> Result<SerpGoogleAiModeLanguagesResponseInfo> {
    let path = "/v3/serp/google/ai_mode/languages".to_string();
    client.get::<SerpGoogleAiModeLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_ai_mode_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleAiModeTaskPostRequestInfo>) -> Result<SerpGoogleAiModeTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleAiModeTaskPostRequestInfo>, SerpGoogleAiModeTaskPostResponseInfo>("/v3/serp/google/ai_mode/task_post", body).await
}

pub async fn google_ai_mode_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleAiModeTasksReadyResponseInfo> {
    let path = "/v3/serp/google/ai_mode/tasks_ready".to_string();
    client.get::<SerpGoogleAiModeTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ai_mode_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleAiModeTasksFixedResponseInfo> {
    let path = "/v3/serp/google/ai_mode/tasks_fixed".to_string();
    client.get::<SerpGoogleAiModeTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_ai_mode_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleAiModeTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/ai_mode/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleAiModeTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_ai_mode_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleAiModeTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/ai_mode/task_get/html/{id}", id = id);
    client.get::<SerpGoogleAiModeTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_ai_mode_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleAiModeLiveAdvancedRequestInfo>) -> Result<SerpGoogleAiModeLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleAiModeLiveAdvancedRequestInfo>, SerpGoogleAiModeLiveAdvancedResponseInfo>("/v3/serp/google/ai_mode/live/advanced", body).await
}

pub async fn google_ai_mode_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleAiModeLiveHtmlRequestInfo>) -> Result<SerpGoogleAiModeLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleAiModeLiveHtmlRequestInfo>, SerpGoogleAiModeLiveHtmlResponseInfo>("/v3/serp/google/ai_mode/live/html", body).await
}

pub async fn google_maps_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleMapsTaskPostRequestInfo>) -> Result<SerpGoogleMapsTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleMapsTaskPostRequestInfo>, SerpGoogleMapsTaskPostResponseInfo>("/v3/serp/google/maps/task_post", body).await
}

pub async fn google_maps_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleMapsTasksReadyResponseInfo> {
    let path = "/v3/serp/google/maps/tasks_ready".to_string();
    client.get::<SerpGoogleMapsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_maps_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleMapsTasksFixedResponseInfo> {
    let path = "/v3/serp/google/maps/tasks_fixed".to_string();
    client.get::<SerpGoogleMapsTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_maps_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleMapsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/maps/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleMapsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_maps_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleMapsLiveAdvancedRequestInfo>) -> Result<SerpGoogleMapsLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleMapsLiveAdvancedRequestInfo>, SerpGoogleMapsLiveAdvancedResponseInfo>("/v3/serp/google/maps/live/advanced", body).await
}

pub async fn google_local_finder_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleLocalFinderTaskPostRequestInfo>) -> Result<SerpGoogleLocalFinderTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleLocalFinderTaskPostRequestInfo>, SerpGoogleLocalFinderTaskPostResponseInfo>("/v3/serp/google/local_finder/task_post", body).await
}

pub async fn google_local_finder_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleLocalFinderTasksReadyResponseInfo> {
    let path = "/v3/serp/google/local_finder/tasks_ready".to_string();
    client.get::<SerpGoogleLocalFinderTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_local_finder_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleLocalFinderTasksFixedResponseInfo> {
    let path = "/v3/serp/google/local_finder/tasks_fixed".to_string();
    client.get::<SerpGoogleLocalFinderTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_local_finder_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleLocalFinderTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/local_finder/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleLocalFinderTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_local_finder_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleLocalFinderTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/local_finder/task_get/html/{id}", id = id);
    client.get::<SerpGoogleLocalFinderTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_local_finder_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleLocalFinderLiveAdvancedRequestInfo>) -> Result<SerpGoogleLocalFinderLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleLocalFinderLiveAdvancedRequestInfo>, SerpGoogleLocalFinderLiveAdvancedResponseInfo>("/v3/serp/google/local_finder/live/advanced", body).await
}

pub async fn google_local_finder_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleLocalFinderLiveHtmlRequestInfo>) -> Result<SerpGoogleLocalFinderLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleLocalFinderLiveHtmlRequestInfo>, SerpGoogleLocalFinderLiveHtmlResponseInfo>("/v3/serp/google/local_finder/live/html", body).await
}

pub async fn google_news_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleNewsTaskPostRequestInfo>) -> Result<SerpGoogleNewsTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleNewsTaskPostRequestInfo>, SerpGoogleNewsTaskPostResponseInfo>("/v3/serp/google/news/task_post", body).await
}

pub async fn google_news_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleNewsTasksReadyResponseInfo> {
    let path = "/v3/serp/google/news/tasks_ready".to_string();
    client.get::<SerpGoogleNewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_news_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleNewsTasksFixedResponseInfo> {
    let path = "/v3/serp/google/news/tasks_fixed".to_string();
    client.get::<SerpGoogleNewsTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_news_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleNewsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/news/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleNewsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_news_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleNewsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/news/task_get/html/{id}", id = id);
    client.get::<SerpGoogleNewsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_news_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleNewsLiveAdvancedRequestInfo>) -> Result<SerpGoogleNewsLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleNewsLiveAdvancedRequestInfo>, SerpGoogleNewsLiveAdvancedResponseInfo>("/v3/serp/google/news/live/advanced", body).await
}

pub async fn google_news_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleNewsLiveHtmlRequestInfo>) -> Result<SerpGoogleNewsLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleNewsLiveHtmlRequestInfo>, SerpGoogleNewsLiveHtmlResponseInfo>("/v3/serp/google/news/live/html", body).await
}

pub async fn google_events_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleEventsTaskPostRequestInfo>) -> Result<SerpGoogleEventsTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleEventsTaskPostRequestInfo>, SerpGoogleEventsTaskPostResponseInfo>("/v3/serp/google/events/task_post", body).await
}

pub async fn google_events_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleEventsTasksReadyResponseInfo> {
    let path = "/v3/serp/google/events/tasks_ready".to_string();
    client.get::<SerpGoogleEventsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_events_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleEventsTasksFixedResponseInfo> {
    let path = "/v3/serp/google/events/tasks_fixed".to_string();
    client.get::<SerpGoogleEventsTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_events_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleEventsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/events/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleEventsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_events_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleEventsLiveAdvancedRequestInfo>) -> Result<SerpGoogleEventsLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleEventsLiveAdvancedRequestInfo>, SerpGoogleEventsLiveAdvancedResponseInfo>("/v3/serp/google/events/live/advanced", body).await
}

pub async fn google_images_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleImagesTaskPostRequestInfo>) -> Result<SerpGoogleImagesTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleImagesTaskPostRequestInfo>, SerpGoogleImagesTaskPostResponseInfo>("/v3/serp/google/images/task_post", body).await
}

pub async fn google_images_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleImagesTasksReadyResponseInfo> {
    let path = "/v3/serp/google/images/tasks_ready".to_string();
    client.get::<SerpGoogleImagesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_images_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleImagesTasksFixedResponseInfo> {
    let path = "/v3/serp/google/images/tasks_fixed".to_string();
    client.get::<SerpGoogleImagesTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_images_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleImagesTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/images/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleImagesTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_images_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleImagesTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/images/task_get/html/{id}", id = id);
    client.get::<SerpGoogleImagesTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_images_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleImagesLiveAdvancedRequestInfo>) -> Result<SerpGoogleImagesLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleImagesLiveAdvancedRequestInfo>, SerpGoogleImagesLiveAdvancedResponseInfo>("/v3/serp/google/images/live/advanced", body).await
}

pub async fn google_images_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleImagesLiveHtmlRequestInfo>) -> Result<SerpGoogleImagesLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleImagesLiveHtmlRequestInfo>, SerpGoogleImagesLiveHtmlResponseInfo>("/v3/serp/google/images/live/html", body).await
}

pub async fn google_search_by_image_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleSearchByImageTaskPostRequestInfo>) -> Result<SerpGoogleSearchByImageTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleSearchByImageTaskPostRequestInfo>, SerpGoogleSearchByImageTaskPostResponseInfo>("/v3/serp/google/search_by_image/task_post", body).await
}

pub async fn google_search_by_image_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleSearchByImageTasksReadyResponseInfo> {
    let path = "/v3/serp/google/search_by_image/tasks_ready".to_string();
    client.get::<SerpGoogleSearchByImageTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_search_by_image_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleSearchByImageTasksFixedResponseInfo> {
    let path = "/v3/serp/google/search_by_image/tasks_fixed".to_string();
    client.get::<SerpGoogleSearchByImageTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_search_by_image_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleSearchByImageTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/search_by_image/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleSearchByImageTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_jobs_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleJobsTaskPostRequestInfo>) -> Result<SerpGoogleJobsTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleJobsTaskPostRequestInfo>, SerpGoogleJobsTaskPostResponseInfo>("/v3/serp/google/jobs/task_post", body).await
}

pub async fn google_jobs_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleJobsTasksReadyResponseInfo> {
    let path = "/v3/serp/google/jobs/tasks_ready".to_string();
    client.get::<SerpGoogleJobsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_jobs_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleJobsTasksFixedResponseInfo> {
    let path = "/v3/serp/google/jobs/tasks_fixed".to_string();
    client.get::<SerpGoogleJobsTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_jobs_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleJobsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/jobs/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleJobsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_jobs_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleJobsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/jobs/task_get/html/{id}", id = id);
    client.get::<SerpGoogleJobsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_autocomplete_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleAutocompleteTaskPostRequestInfo>) -> Result<SerpGoogleAutocompleteTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleAutocompleteTaskPostRequestInfo>, SerpGoogleAutocompleteTaskPostResponseInfo>("/v3/serp/google/autocomplete/task_post", body).await
}

pub async fn google_autocomplete_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleAutocompleteTasksReadyResponseInfo> {
    let path = "/v3/serp/google/autocomplete/tasks_ready".to_string();
    client.get::<SerpGoogleAutocompleteTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_autocomplete_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleAutocompleteTasksFixedResponseInfo> {
    let path = "/v3/serp/google/autocomplete/tasks_fixed".to_string();
    client.get::<SerpGoogleAutocompleteTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_autocomplete_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleAutocompleteTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/autocomplete/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleAutocompleteTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_autocomplete_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleAutocompleteLiveAdvancedRequestInfo>) -> Result<SerpGoogleAutocompleteLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleAutocompleteLiveAdvancedRequestInfo>, SerpGoogleAutocompleteLiveAdvancedResponseInfo>("/v3/serp/google/autocomplete/live/advanced", body).await
}

pub async fn google_dataset_search_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleDatasetSearchTaskPostRequestInfo>) -> Result<SerpGoogleDatasetSearchTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleDatasetSearchTaskPostRequestInfo>, SerpGoogleDatasetSearchTaskPostResponseInfo>("/v3/serp/google/dataset_search/task_post", body).await
}

pub async fn google_dataset_search_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleDatasetSearchTasksReadyResponseInfo> {
    let path = "/v3/serp/google/dataset_search/tasks_ready".to_string();
    client.get::<SerpGoogleDatasetSearchTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_search_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleDatasetSearchTasksFixedResponseInfo> {
    let path = "/v3/serp/google/dataset_search/tasks_fixed".to_string();
    client.get::<SerpGoogleDatasetSearchTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_search_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleDatasetSearchTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/dataset_search/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleDatasetSearchTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_search_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleDatasetSearchLiveAdvancedRequestInfo>) -> Result<SerpGoogleDatasetSearchLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleDatasetSearchLiveAdvancedRequestInfo>, SerpGoogleDatasetSearchLiveAdvancedResponseInfo>("/v3/serp/google/dataset_search/live/advanced", body).await
}

pub async fn google_dataset_info_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleDatasetInfoTaskPostRequestInfo>) -> Result<SerpGoogleDatasetInfoTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleDatasetInfoTaskPostRequestInfo>, SerpGoogleDatasetInfoTaskPostResponseInfo>("/v3/serp/google/dataset_info/task_post", body).await
}

pub async fn google_dataset_info_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleDatasetInfoTasksReadyResponseInfo> {
    let path = "/v3/serp/google/dataset_info/tasks_ready".to_string();
    client.get::<SerpGoogleDatasetInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_info_tasks_fixed(client: &DataForSeoClient) -> Result<SerpGoogleDatasetInfoTasksFixedResponseInfo> {
    let path = "/v3/serp/google/dataset_info/tasks_fixed".to_string();
    client.get::<SerpGoogleDatasetInfoTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleDatasetInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/dataset_info/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleDatasetInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_dataset_info_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleDatasetInfoLiveAdvancedRequestInfo>) -> Result<SerpGoogleDatasetInfoLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleDatasetInfoLiveAdvancedRequestInfo>, SerpGoogleDatasetInfoLiveAdvancedResponseInfo>("/v3/serp/google/dataset_info/live/advanced", body).await
}

pub async fn google_ads_advertisers_locations(client: &DataForSeoClient) -> Result<SerpGoogleAdsAdvertisersLocationsResponseInfo> {
    let path = "/v3/serp/google/ads_advertisers/locations".to_string();
    client.get::<SerpGoogleAdsAdvertisersLocationsResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_advertisers_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleAdsAdvertisersTaskPostRequestInfo>) -> Result<SerpGoogleAdsAdvertisersTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleAdsAdvertisersTaskPostRequestInfo>, SerpGoogleAdsAdvertisersTaskPostResponseInfo>("/v3/serp/google/ads_advertisers/task_post", body).await
}

pub async fn google_ads_advertisers_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleAdsAdvertisersTasksReadyResponseInfo> {
    let path = "/v3/serp/google/ads_advertisers/tasks_ready".to_string();
    client.get::<SerpGoogleAdsAdvertisersTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_advertisers_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleAdsAdvertisersTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/ads_advertisers/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleAdsAdvertisersTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_locations(client: &DataForSeoClient) -> Result<SerpGoogleAdsSearchLocationsResponseInfo> {
    let path = "/v3/serp/google/ads_search/locations".to_string();
    client.get::<SerpGoogleAdsSearchLocationsResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleAdsSearchTaskPostRequestInfo>) -> Result<SerpGoogleAdsSearchTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleAdsSearchTaskPostRequestInfo>, SerpGoogleAdsSearchTaskPostResponseInfo>("/v3/serp/google/ads_search/task_post", body).await
}

pub async fn google_ads_search_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleAdsSearchTasksReadyResponseInfo> {
    let path = "/v3/serp/google/ads_search/tasks_ready".to_string();
    client.get::<SerpGoogleAdsSearchTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleAdsSearchTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/ads_search/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleAdsSearchTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn bing_locations(client: &DataForSeoClient) -> Result<SerpBingLocationsResponseInfo> {
    let path = "/v3/serp/bing/locations".to_string();
    client.get::<SerpBingLocationsResponseInfo>(
        &path
    ).await
}

pub async fn bing_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpBingLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/bing/locations/{country}", country = country);
    client.get::<SerpBingLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn bing_languages(client: &DataForSeoClient) -> Result<SerpBingLanguagesResponseInfo> {
    let path = "/v3/serp/bing/languages".to_string();
    client.get::<SerpBingLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpBingOrganicTaskPostRequestInfo>) -> Result<SerpBingOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpBingOrganicTaskPostRequestInfo>, SerpBingOrganicTaskPostResponseInfo>("/v3/serp/bing/organic/task_post", body).await
}

pub async fn bing_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpBingOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/bing/organic/tasks_ready".to_string();
    client.get::<SerpBingOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpBingOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/bing/organic/tasks_fixed".to_string();
    client.get::<SerpBingOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpBingOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/bing/organic/task_get/regular/{id}", id = id);
    client.get::<SerpBingOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpBingOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/bing/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpBingOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpBingOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/bing/organic/task_get/html/{id}", id = id);
    client.get::<SerpBingOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn bing_organic_live_regular(client: &DataForSeoClient, body: &Vec<SerpBingOrganicLiveRegularRequestInfo>) -> Result<SerpBingOrganicLiveRegularResponseInfo> {
    client.post::<Vec<SerpBingOrganicLiveRegularRequestInfo>, SerpBingOrganicLiveRegularResponseInfo>("/v3/serp/bing/organic/live/regular", body).await
}

pub async fn bing_organic_live_advanced(client: &DataForSeoClient, body: &Vec<SerpBingOrganicLiveAdvancedRequestInfo>) -> Result<SerpBingOrganicLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpBingOrganicLiveAdvancedRequestInfo>, SerpBingOrganicLiveAdvancedResponseInfo>("/v3/serp/bing/organic/live/advanced", body).await
}

pub async fn bing_organic_live_html(client: &DataForSeoClient, body: &Vec<SerpBingOrganicLiveHtmlRequestInfo>) -> Result<SerpBingOrganicLiveHtmlResponseInfo> {
    client.post::<Vec<SerpBingOrganicLiveHtmlRequestInfo>, SerpBingOrganicLiveHtmlResponseInfo>("/v3/serp/bing/organic/live/html", body).await
}

pub async fn bing_local_pack_task_post(client: &DataForSeoClient, body: &Vec<SerpBingLocalPackTaskPostRequestInfo>) -> Result<SerpBingLocalPackTaskPostResponseInfo> {
    client.post::<Vec<SerpBingLocalPackTaskPostRequestInfo>, SerpBingLocalPackTaskPostResponseInfo>("/v3/serp/bing/local_pack/task_post", body).await
}

pub async fn bing_local_pack_tasks_ready(client: &DataForSeoClient) -> Result<SerpBingLocalPackTasksReadyResponseInfo> {
    let path = "/v3/serp/bing/local_pack/tasks_ready".to_string();
    client.get::<SerpBingLocalPackTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_local_pack_tasks_fixed(client: &DataForSeoClient) -> Result<SerpBingLocalPackTasksFixedResponseInfo> {
    let path = "/v3/serp/bing/local_pack/tasks_fixed".to_string();
    client.get::<SerpBingLocalPackTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn bing_local_pack_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpBingLocalPackTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/bing/local_pack/task_get/regular/{id}", id = id);
    client.get::<SerpBingLocalPackTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn bing_local_pack_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpBingLocalPackTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/bing/local_pack/task_get/html/{id}", id = id);
    client.get::<SerpBingLocalPackTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn bing_local_pack_live_regular(client: &DataForSeoClient, body: &Vec<SerpBingLocalPackLiveRegularRequestInfo>) -> Result<SerpBingLocalPackLiveRegularResponseInfo> {
    client.post::<Vec<SerpBingLocalPackLiveRegularRequestInfo>, SerpBingLocalPackLiveRegularResponseInfo>("/v3/serp/bing/local_pack/live/regular", body).await
}

pub async fn bing_local_pack_live_html(client: &DataForSeoClient, body: &Vec<SerpBingLocalPackLiveHtmlRequestInfo>) -> Result<SerpBingLocalPackLiveHtmlResponseInfo> {
    client.post::<Vec<SerpBingLocalPackLiveHtmlRequestInfo>, SerpBingLocalPackLiveHtmlResponseInfo>("/v3/serp/bing/local_pack/live/html", body).await
}

pub async fn youtube_locations(client: &DataForSeoClient) -> Result<SerpYoutubeLocationsResponseInfo> {
    let path = "/v3/serp/youtube/locations".to_string();
    client.get::<SerpYoutubeLocationsResponseInfo>(
        &path
    ).await
}

pub async fn youtube_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpYoutubeLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/youtube/locations/{country}", country = country);
    client.get::<SerpYoutubeLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn youtube_languages(client: &DataForSeoClient) -> Result<SerpYoutubeLanguagesResponseInfo> {
    let path = "/v3/serp/youtube/languages".to_string();
    client.get::<SerpYoutubeLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_info_task_post(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoInfoTaskPostRequestInfo>) -> Result<SerpYoutubeVideoInfoTaskPostResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoInfoTaskPostRequestInfo>, SerpYoutubeVideoInfoTaskPostResponseInfo>("/v3/serp/youtube/video_info/task_post", body).await
}

pub async fn youtube_video_info_tasks_ready(client: &DataForSeoClient) -> Result<SerpYoutubeVideoInfoTasksReadyResponseInfo> {
    let path = "/v3/serp/youtube/video_info/tasks_ready".to_string();
    client.get::<SerpYoutubeVideoInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_info_tasks_fixed(client: &DataForSeoClient) -> Result<SerpYoutubeVideoInfoTasksFixedResponseInfo> {
    let path = "/v3/serp/youtube/video_info/tasks_fixed".to_string();
    client.get::<SerpYoutubeVideoInfoTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpYoutubeVideoInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/youtube/video_info/task_get/advanced/{id}", id = id);
    client.get::<SerpYoutubeVideoInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_info_live_advanced(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoInfoLiveAdvancedRequestInfo>) -> Result<SerpYoutubeVideoInfoLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoInfoLiveAdvancedRequestInfo>, SerpYoutubeVideoInfoLiveAdvancedResponseInfo>("/v3/serp/youtube/video_info/live/advanced", body).await
}

pub async fn youtube_video_subtitles_task_post(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoSubtitlesTaskPostRequestInfo>) -> Result<SerpYoutubeVideoSubtitlesTaskPostResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoSubtitlesTaskPostRequestInfo>, SerpYoutubeVideoSubtitlesTaskPostResponseInfo>("/v3/serp/youtube/video_subtitles/task_post", body).await
}

pub async fn youtube_video_subtitles_tasks_ready(client: &DataForSeoClient) -> Result<SerpYoutubeVideoSubtitlesTasksReadyResponseInfo> {
    let path = "/v3/serp/youtube/video_subtitles/tasks_ready".to_string();
    client.get::<SerpYoutubeVideoSubtitlesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_subtitles_tasks_fixed(client: &DataForSeoClient) -> Result<SerpYoutubeVideoSubtitlesTasksFixedResponseInfo> {
    let path = "/v3/serp/youtube/video_subtitles/tasks_fixed".to_string();
    client.get::<SerpYoutubeVideoSubtitlesTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_subtitles_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpYoutubeVideoSubtitlesTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/youtube/video_subtitles/task_get/advanced/{id}", id = id);
    client.get::<SerpYoutubeVideoSubtitlesTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_subtitles_live_advanced(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoSubtitlesLiveAdvancedRequestInfo>) -> Result<SerpYoutubeVideoSubtitlesLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoSubtitlesLiveAdvancedRequestInfo>, SerpYoutubeVideoSubtitlesLiveAdvancedResponseInfo>("/v3/serp/youtube/video_subtitles/live/advanced", body).await
}

pub async fn youtube_video_comments_task_post(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoCommentsTaskPostRequestInfo>) -> Result<SerpYoutubeVideoCommentsTaskPostResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoCommentsTaskPostRequestInfo>, SerpYoutubeVideoCommentsTaskPostResponseInfo>("/v3/serp/youtube/video_comments/task_post", body).await
}

pub async fn youtube_video_comments_tasks_ready(client: &DataForSeoClient) -> Result<SerpYoutubeVideoCommentsTasksReadyResponseInfo> {
    let path = "/v3/serp/youtube/video_comments/tasks_ready".to_string();
    client.get::<SerpYoutubeVideoCommentsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_comments_tasks_fixed(client: &DataForSeoClient) -> Result<SerpYoutubeVideoCommentsTasksFixedResponseInfo> {
    let path = "/v3/serp/youtube/video_comments/tasks_fixed".to_string();
    client.get::<SerpYoutubeVideoCommentsTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_comments_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpYoutubeVideoCommentsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/youtube/video_comments/task_get/advanced/{id}", id = id);
    client.get::<SerpYoutubeVideoCommentsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn youtube_video_comments_live_advanced(client: &DataForSeoClient, body: &Vec<SerpYoutubeVideoCommentsLiveAdvancedRequestInfo>) -> Result<SerpYoutubeVideoCommentsLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpYoutubeVideoCommentsLiveAdvancedRequestInfo>, SerpYoutubeVideoCommentsLiveAdvancedResponseInfo>("/v3/serp/youtube/video_comments/live/advanced", body).await
}

pub async fn yahoo_locations(client: &DataForSeoClient) -> Result<SerpYahooLocationsResponseInfo> {
    let path = "/v3/serp/yahoo/locations".to_string();
    client.get::<SerpYahooLocationsResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpYahooLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/yahoo/locations/{country}", country = country);
    client.get::<SerpYahooLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_languages(client: &DataForSeoClient) -> Result<SerpYahooLanguagesResponseInfo> {
    let path = "/v3/serp/yahoo/languages".to_string();
    client.get::<SerpYahooLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpYahooOrganicTaskPostRequestInfo>) -> Result<SerpYahooOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpYahooOrganicTaskPostRequestInfo>, SerpYahooOrganicTaskPostResponseInfo>("/v3/serp/yahoo/organic/task_post", body).await
}

pub async fn yahoo_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpYahooOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/yahoo/organic/tasks_ready".to_string();
    client.get::<SerpYahooOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpYahooOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/yahoo/organic/tasks_fixed".to_string();
    client.get::<SerpYahooOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpYahooOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/yahoo/organic/task_get/regular/{id}", id = id);
    client.get::<SerpYahooOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpYahooOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/yahoo/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpYahooOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpYahooOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/yahoo/organic/task_get/html/{id}", id = id);
    client.get::<SerpYahooOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn yahoo_organic_live_regular(client: &DataForSeoClient, body: &Vec<SerpYahooOrganicLiveRegularRequestInfo>) -> Result<SerpYahooOrganicLiveRegularResponseInfo> {
    client.post::<Vec<SerpYahooOrganicLiveRegularRequestInfo>, SerpYahooOrganicLiveRegularResponseInfo>("/v3/serp/yahoo/organic/live/regular", body).await
}

pub async fn yahoo_organic_live_advanced(client: &DataForSeoClient, body: &Vec<SerpYahooOrganicLiveAdvancedRequestInfo>) -> Result<SerpYahooOrganicLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpYahooOrganicLiveAdvancedRequestInfo>, SerpYahooOrganicLiveAdvancedResponseInfo>("/v3/serp/yahoo/organic/live/advanced", body).await
}

pub async fn yahoo_organic_live_html(client: &DataForSeoClient, body: &Vec<SerpYahooOrganicLiveHtmlRequestInfo>) -> Result<SerpYahooOrganicLiveHtmlResponseInfo> {
    client.post::<Vec<SerpYahooOrganicLiveHtmlRequestInfo>, SerpYahooOrganicLiveHtmlResponseInfo>("/v3/serp/yahoo/organic/live/html", body).await
}

pub async fn baidu_locations(client: &DataForSeoClient) -> Result<SerpBaiduLocationsResponseInfo> {
    let path = "/v3/serp/baidu/locations".to_string();
    client.get::<SerpBaiduLocationsResponseInfo>(
        &path
    ).await
}

pub async fn baidu_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpBaiduLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/baidu/locations/{country}", country = country);
    client.get::<SerpBaiduLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn baidu_languages(client: &DataForSeoClient) -> Result<SerpBaiduLanguagesResponseInfo> {
    let path = "/v3/serp/baidu/languages".to_string();
    client.get::<SerpBaiduLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn baidu_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpBaiduOrganicTaskPostRequestInfo>) -> Result<SerpBaiduOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpBaiduOrganicTaskPostRequestInfo>, SerpBaiduOrganicTaskPostResponseInfo>("/v3/serp/baidu/organic/task_post", body).await
}

pub async fn baidu_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpBaiduOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/baidu/organic/tasks_ready".to_string();
    client.get::<SerpBaiduOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn baidu_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpBaiduOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/baidu/organic/tasks_fixed".to_string();
    client.get::<SerpBaiduOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn baidu_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpBaiduOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/baidu/organic/task_get/regular/{id}", id = id);
    client.get::<SerpBaiduOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn baidu_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpBaiduOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/baidu/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpBaiduOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn baidu_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpBaiduOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/baidu/organic/task_get/html/{id}", id = id);
    client.get::<SerpBaiduOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn naver_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpNaverOrganicTaskPostRequestInfo>) -> Result<SerpNaverOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpNaverOrganicTaskPostRequestInfo>, SerpNaverOrganicTaskPostResponseInfo>("/v3/serp/naver/organic/task_post", body).await
}

pub async fn naver_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpNaverOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/naver/organic/tasks_ready".to_string();
    client.get::<SerpNaverOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn naver_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpNaverOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/naver/organic/tasks_fixed".to_string();
    client.get::<SerpNaverOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn naver_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpNaverOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/naver/organic/task_get/regular/{id}", id = id);
    client.get::<SerpNaverOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn naver_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpNaverOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/naver/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpNaverOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn naver_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpNaverOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/naver/organic/task_get/html/{id}", id = id);
    client.get::<SerpNaverOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn seznam_locations(client: &DataForSeoClient) -> Result<SerpSeznamLocationsResponseInfo> {
    let path = "/v3/serp/seznam/locations".to_string();
    client.get::<SerpSeznamLocationsResponseInfo>(
        &path
    ).await
}

pub async fn seznam_locations_country(client: &DataForSeoClient, country: &str) -> Result<SerpSeznamLocationsCountryResponseInfo> {
    let path = format!("/v3/serp/seznam/locations/{country}", country = country);
    client.get::<SerpSeznamLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn seznam_languages(client: &DataForSeoClient) -> Result<SerpSeznamLanguagesResponseInfo> {
    let path = "/v3/serp/seznam/languages".to_string();
    client.get::<SerpSeznamLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn seznam_organic_task_post(client: &DataForSeoClient, body: &Vec<SerpSeznamOrganicTaskPostRequestInfo>) -> Result<SerpSeznamOrganicTaskPostResponseInfo> {
    client.post::<Vec<SerpSeznamOrganicTaskPostRequestInfo>, SerpSeznamOrganicTaskPostResponseInfo>("/v3/serp/seznam/organic/task_post", body).await
}

pub async fn seznam_organic_tasks_ready(client: &DataForSeoClient) -> Result<SerpSeznamOrganicTasksReadyResponseInfo> {
    let path = "/v3/serp/seznam/organic/tasks_ready".to_string();
    client.get::<SerpSeznamOrganicTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn seznam_organic_tasks_fixed(client: &DataForSeoClient) -> Result<SerpSeznamOrganicTasksFixedResponseInfo> {
    let path = "/v3/serp/seznam/organic/tasks_fixed".to_string();
    client.get::<SerpSeznamOrganicTasksFixedResponseInfo>(
        &path
    ).await
}

pub async fn seznam_organic_task_get_regular(client: &DataForSeoClient, id: &str) -> Result<SerpSeznamOrganicTaskGetRegularResponseInfo> {
    let path = format!("/v3/serp/seznam/organic/task_get/regular/{id}", id = id);
    client.get::<SerpSeznamOrganicTaskGetRegularResponseInfo>(
        &path
    ).await
}

pub async fn seznam_organic_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpSeznamOrganicTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/seznam/organic/task_get/advanced/{id}", id = id);
    client.get::<SerpSeznamOrganicTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn seznam_organic_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpSeznamOrganicTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/seznam/organic/task_get/html/{id}", id = id);
    client.get::<SerpSeznamOrganicTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_explore_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceExploreTaskPostRequestInfo>) -> Result<SerpGoogleFinanceExploreTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceExploreTaskPostRequestInfo>, SerpGoogleFinanceExploreTaskPostResponseInfo>("/v3/serp/google/finance_explore/task_post", body).await
}

pub async fn google_finance_explore_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleFinanceExploreTasksReadyResponseInfo> {
    let path = "/v3/serp/google/finance_explore/tasks_ready".to_string();
    client.get::<SerpGoogleFinanceExploreTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_explore_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceExploreTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/finance_explore/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleFinanceExploreTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_explore_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceExploreTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/finance_explore/task_get/html/{id}", id = id);
    client.get::<SerpGoogleFinanceExploreTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_explore_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceExploreLiveAdvancedRequestInfo>) -> Result<SerpGoogleFinanceExploreLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceExploreLiveAdvancedRequestInfo>, SerpGoogleFinanceExploreLiveAdvancedResponseInfo>("/v3/serp/google/finance_explore/live/advanced", body).await
}

pub async fn google_finance_explore_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceExploreLiveHtmlRequestInfo>) -> Result<SerpGoogleFinanceExploreLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceExploreLiveHtmlRequestInfo>, SerpGoogleFinanceExploreLiveHtmlResponseInfo>("/v3/serp/google/finance_explore/live/html", body).await
}

pub async fn google_finance_markets_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceMarketsTaskPostRequestInfo>) -> Result<SerpGoogleFinanceMarketsTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceMarketsTaskPostRequestInfo>, SerpGoogleFinanceMarketsTaskPostResponseInfo>("/v3/serp/google/finance_markets/task_post", body).await
}

pub async fn google_finance_markets_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleFinanceMarketsTasksReadyResponseInfo> {
    let path = "/v3/serp/google/finance_markets/tasks_ready".to_string();
    client.get::<SerpGoogleFinanceMarketsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_markets_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceMarketsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/finance_markets/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleFinanceMarketsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_markets_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceMarketsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/finance_markets/task_get/html/{id}", id = id);
    client.get::<SerpGoogleFinanceMarketsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_markets_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceMarketsLiveAdvancedRequestInfo>) -> Result<SerpGoogleFinanceMarketsLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceMarketsLiveAdvancedRequestInfo>, SerpGoogleFinanceMarketsLiveAdvancedResponseInfo>("/v3/serp/google/finance_markets/live/advanced", body).await
}

pub async fn google_finance_markets_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceMarketsLiveHtmlRequestInfo>) -> Result<SerpGoogleFinanceMarketsLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceMarketsLiveHtmlRequestInfo>, SerpGoogleFinanceMarketsLiveHtmlResponseInfo>("/v3/serp/google/finance_markets/live/html", body).await
}

pub async fn google_finance_quote_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceQuoteTaskPostRequestInfo>) -> Result<SerpGoogleFinanceQuoteTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceQuoteTaskPostRequestInfo>, SerpGoogleFinanceQuoteTaskPostResponseInfo>("/v3/serp/google/finance_quote/task_post", body).await
}

pub async fn google_finance_quote_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleFinanceQuoteTasksReadyResponseInfo> {
    let path = "/v3/serp/google/finance_quote/tasks_ready".to_string();
    client.get::<SerpGoogleFinanceQuoteTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_quote_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceQuoteTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/finance_quote/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleFinanceQuoteTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_quote_task_get_html(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceQuoteTaskGetHtmlResponseInfo> {
    let path = format!("/v3/serp/google/finance_quote/task_get/html/{id}", id = id);
    client.get::<SerpGoogleFinanceQuoteTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_quote_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceQuoteLiveAdvancedRequestInfo>) -> Result<SerpGoogleFinanceQuoteLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceQuoteLiveAdvancedRequestInfo>, SerpGoogleFinanceQuoteLiveAdvancedResponseInfo>("/v3/serp/google/finance_quote/live/advanced", body).await
}

pub async fn google_finance_quote_live_html(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceQuoteLiveHtmlRequestInfo>) -> Result<SerpGoogleFinanceQuoteLiveHtmlResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceQuoteLiveHtmlRequestInfo>, SerpGoogleFinanceQuoteLiveHtmlResponseInfo>("/v3/serp/google/finance_quote/live/html", body).await
}

pub async fn google_finance_ticker_search_task_post(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceTickerSearchTaskPostRequestInfo>) -> Result<SerpGoogleFinanceTickerSearchTaskPostResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceTickerSearchTaskPostRequestInfo>, SerpGoogleFinanceTickerSearchTaskPostResponseInfo>("/v3/serp/google/finance_ticker_search/task_post", body).await
}

pub async fn google_finance_ticker_search_tasks_ready(client: &DataForSeoClient) -> Result<SerpGoogleFinanceTickerSearchTasksReadyResponseInfo> {
    let path = "/v3/serp/google/finance_ticker_search/tasks_ready".to_string();
    client.get::<SerpGoogleFinanceTickerSearchTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_ticker_search_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<SerpGoogleFinanceTickerSearchTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/serp/google/finance_ticker_search/task_get/advanced/{id}", id = id);
    client.get::<SerpGoogleFinanceTickerSearchTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_finance_ticker_search_live_advanced(client: &DataForSeoClient, body: &Vec<SerpGoogleFinanceTickerSearchLiveAdvancedRequestInfo>) -> Result<SerpGoogleFinanceTickerSearchLiveAdvancedResponseInfo> {
    client.post::<Vec<SerpGoogleFinanceTickerSearchLiveAdvancedRequestInfo>, SerpGoogleFinanceTickerSearchLiveAdvancedResponseInfo>("/v3/serp/google/finance_ticker_search/live/advanced", body).await
}

pub async fn dataforseo_labs_id_list(client: &DataForSeoClient, body: &Vec<DataforseoLabsIdListRequestInfo>) -> Result<DataforseoLabsIdListResponseInfo> {
    client.post::<Vec<DataforseoLabsIdListRequestInfo>, DataforseoLabsIdListResponseInfo>("/v3/dataforseo_labs/id_list", body).await
}

pub async fn status(client: &DataForSeoClient) -> Result<DataforseoLabsStatusResponseInfo> {
    let path = "/v3/dataforseo_labs/status".to_string();
    client.get::<DataforseoLabsStatusResponseInfo>(
        &path
    ).await
}

pub async fn dataforseo_labs_errors(client: &DataForSeoClient, body: &Vec<DataforseoLabsErrorsRequestInfo>) -> Result<DataforseoLabsErrorsResponseInfo> {
    client.post::<Vec<DataforseoLabsErrorsRequestInfo>, DataforseoLabsErrorsResponseInfo>("/v3/dataforseo_labs/errors", body).await
}

pub async fn available_filters(client: &DataForSeoClient) -> Result<DataforseoLabsAvailableFiltersResponseInfo> {
    let path = "/v3/dataforseo_labs/available_filters".to_string();
    client.get::<DataforseoLabsAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn locations_and_languages(client: &DataForSeoClient) -> Result<DataforseoLabsLocationsAndLanguagesResponseInfo> {
    let path = "/v3/dataforseo_labs/locations_and_languages".to_string();
    client.get::<DataforseoLabsLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn categories(client: &DataForSeoClient) -> Result<DataforseoLabsCategoriesResponseInfo> {
    let path = "/v3/dataforseo_labs/categories".to_string();
    client.get::<DataforseoLabsCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn google_available_history(client: &DataForSeoClient) -> Result<DataforseoLabsGoogleAvailableHistoryResponseInfo> {
    let path = "/v3/dataforseo_labs/google/available_history".to_string();
    client.get::<DataforseoLabsGoogleAvailableHistoryResponseInfo>(
        &path
    ).await
}

pub async fn google_keywords_for_site_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordsForSiteLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordsForSiteLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordsForSiteLiveRequestInfo>, DataforseoLabsGoogleKeywordsForSiteLiveResponseInfo>("/v3/dataforseo_labs/google/keywords_for_site/live", body).await
}

pub async fn google_related_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleRelatedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsGoogleRelatedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleRelatedKeywordsLiveRequestInfo>, DataforseoLabsGoogleRelatedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/google/related_keywords/live", body).await
}

pub async fn google_keyword_suggestions_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordSuggestionsLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordSuggestionsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordSuggestionsLiveRequestInfo>, DataforseoLabsGoogleKeywordSuggestionsLiveResponseInfo>("/v3/dataforseo_labs/google/keyword_suggestions/live", body).await
}

pub async fn google_keyword_ideas_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordIdeasLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordIdeasLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordIdeasLiveRequestInfo>, DataforseoLabsGoogleKeywordIdeasLiveResponseInfo>("/v3/dataforseo_labs/google/keyword_ideas/live", body).await
}

pub async fn google_bulk_keyword_difficulty_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleBulkKeywordDifficultyLiveRequestInfo>) -> Result<DataforseoLabsGoogleBulkKeywordDifficultyLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleBulkKeywordDifficultyLiveRequestInfo>, DataforseoLabsGoogleBulkKeywordDifficultyLiveResponseInfo>("/v3/dataforseo_labs/google/bulk_keyword_difficulty/live", body).await
}

pub async fn google_search_intent_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleSearchIntentLiveRequestInfo>) -> Result<DataforseoLabsGoogleSearchIntentLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleSearchIntentLiveRequestInfo>, DataforseoLabsGoogleSearchIntentLiveResponseInfo>("/v3/dataforseo_labs/google/search_intent/live", body).await
}

pub async fn google_categories_for_keywords_languages(client: &DataForSeoClient) -> Result<DataforseoLabsGoogleCategoriesForKeywordsLanguagesResponseInfo> {
    let path = "/v3/dataforseo_labs/google/categories_for_keywords/languages".to_string();
    client.get::<DataforseoLabsGoogleCategoriesForKeywordsLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_categories_for_domain_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleCategoriesForDomainLiveRequestInfo>) -> Result<DataforseoLabsGoogleCategoriesForDomainLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleCategoriesForDomainLiveRequestInfo>, DataforseoLabsGoogleCategoriesForDomainLiveResponseInfo>("/v3/dataforseo_labs/google/categories_for_domain/live", body).await
}

pub async fn google_categories_for_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleCategoriesForKeywordsLiveRequestInfo>) -> Result<DataforseoLabsGoogleCategoriesForKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleCategoriesForKeywordsLiveRequestInfo>, DataforseoLabsGoogleCategoriesForKeywordsLiveResponseInfo>("/v3/dataforseo_labs/google/categories_for_keywords/live", body).await
}

pub async fn google_keywords_for_categories_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordsForCategoriesLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordsForCategoriesLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordsForCategoriesLiveRequestInfo>, DataforseoLabsGoogleKeywordsForCategoriesLiveResponseInfo>("/v3/dataforseo_labs/google/keywords_for_categories/live", body).await
}

pub async fn google_domain_metrics_by_categories_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleDomainMetricsByCategoriesLiveRequestInfo>) -> Result<DataforseoLabsGoogleDomainMetricsByCategoriesLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleDomainMetricsByCategoriesLiveRequestInfo>, DataforseoLabsGoogleDomainMetricsByCategoriesLiveResponseInfo>("/v3/dataforseo_labs/google/domain_metrics_by_categories/live", body).await
}

pub async fn google_top_searches_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleTopSearchesLiveRequestInfo>) -> Result<DataforseoLabsGoogleTopSearchesLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleTopSearchesLiveRequestInfo>, DataforseoLabsGoogleTopSearchesLiveResponseInfo>("/v3/dataforseo_labs/google/top_searches/live", body).await
}

pub async fn google_domain_whois_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleDomainWhoisOverviewLiveRequestInfo>) -> Result<DataforseoLabsGoogleDomainWhoisOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleDomainWhoisOverviewLiveRequestInfo>, DataforseoLabsGoogleDomainWhoisOverviewLiveResponseInfo>("/v3/dataforseo_labs/google/domain_whois_overview/live", body).await
}

pub async fn google_ranked_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleRankedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsGoogleRankedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleRankedKeywordsLiveRequestInfo>, DataforseoLabsGoogleRankedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/google/ranked_keywords/live", body).await
}

pub async fn google_serp_competitors_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleSerpCompetitorsLiveRequestInfo>) -> Result<DataforseoLabsGoogleSerpCompetitorsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleSerpCompetitorsLiveRequestInfo>, DataforseoLabsGoogleSerpCompetitorsLiveResponseInfo>("/v3/dataforseo_labs/google/serp_competitors/live", body).await
}

pub async fn google_competitors_domain_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleCompetitorsDomainLiveRequestInfo>) -> Result<DataforseoLabsGoogleCompetitorsDomainLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleCompetitorsDomainLiveRequestInfo>, DataforseoLabsGoogleCompetitorsDomainLiveResponseInfo>("/v3/dataforseo_labs/google/competitors_domain/live", body).await
}

pub async fn google_domain_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleDomainIntersectionLiveRequestInfo>) -> Result<DataforseoLabsGoogleDomainIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleDomainIntersectionLiveRequestInfo>, DataforseoLabsGoogleDomainIntersectionLiveResponseInfo>("/v3/dataforseo_labs/google/domain_intersection/live", body).await
}

pub async fn google_subdomains_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleSubdomainsLiveRequestInfo>) -> Result<DataforseoLabsGoogleSubdomainsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleSubdomainsLiveRequestInfo>, DataforseoLabsGoogleSubdomainsLiveResponseInfo>("/v3/dataforseo_labs/google/subdomains/live", body).await
}

pub async fn google_relevant_pages_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleRelevantPagesLiveRequestInfo>) -> Result<DataforseoLabsGoogleRelevantPagesLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleRelevantPagesLiveRequestInfo>, DataforseoLabsGoogleRelevantPagesLiveResponseInfo>("/v3/dataforseo_labs/google/relevant_pages/live", body).await
}

pub async fn google_domain_rank_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleDomainRankOverviewLiveRequestInfo>) -> Result<DataforseoLabsGoogleDomainRankOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleDomainRankOverviewLiveRequestInfo>, DataforseoLabsGoogleDomainRankOverviewLiveResponseInfo>("/v3/dataforseo_labs/google/domain_rank_overview/live", body).await
}

pub async fn google_historical_serps_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleHistoricalSerpsLiveRequestInfo>) -> Result<DataforseoLabsGoogleHistoricalSerpsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleHistoricalSerpsLiveRequestInfo>, DataforseoLabsGoogleHistoricalSerpsLiveResponseInfo>("/v3/dataforseo_labs/google/historical_serps/live", body).await
}

pub async fn google_historical_rank_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleHistoricalRankOverviewLiveRequestInfo>) -> Result<DataforseoLabsGoogleHistoricalRankOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleHistoricalRankOverviewLiveRequestInfo>, DataforseoLabsGoogleHistoricalRankOverviewLiveResponseInfo>("/v3/dataforseo_labs/google/historical_rank_overview/live", body).await
}

pub async fn google_page_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGooglePageIntersectionLiveRequestInfo>) -> Result<DataforseoLabsGooglePageIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGooglePageIntersectionLiveRequestInfo>, DataforseoLabsGooglePageIntersectionLiveResponseInfo>("/v3/dataforseo_labs/google/page_intersection/live", body).await
}

pub async fn google_bulk_traffic_estimation_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleBulkTrafficEstimationLiveRequestInfo>) -> Result<DataforseoLabsGoogleBulkTrafficEstimationLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleBulkTrafficEstimationLiveRequestInfo>, DataforseoLabsGoogleBulkTrafficEstimationLiveResponseInfo>("/v3/dataforseo_labs/google/bulk_traffic_estimation/live", body).await
}

pub async fn google_historical_bulk_traffic_estimation_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleHistoricalBulkTrafficEstimationLiveRequestInfo>) -> Result<DataforseoLabsGoogleHistoricalBulkTrafficEstimationLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleHistoricalBulkTrafficEstimationLiveRequestInfo>, DataforseoLabsGoogleHistoricalBulkTrafficEstimationLiveResponseInfo>("/v3/dataforseo_labs/google/historical_bulk_traffic_estimation/live", body).await
}

pub async fn google_historical_keyword_data_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleHistoricalKeywordDataLiveRequestInfo>) -> Result<DataforseoLabsGoogleHistoricalKeywordDataLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleHistoricalKeywordDataLiveRequestInfo>, DataforseoLabsGoogleHistoricalKeywordDataLiveResponseInfo>("/v3/dataforseo_labs/google/historical_keyword_data/live", body).await
}

pub async fn google_keyword_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordOverviewLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordOverviewLiveRequestInfo>, DataforseoLabsGoogleKeywordOverviewLiveResponseInfo>("/v3/dataforseo_labs/google/keyword_overview/live", body).await
}

pub async fn amazon_bulk_search_volume_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonBulkSearchVolumeLiveRequestInfo>) -> Result<DataforseoLabsAmazonBulkSearchVolumeLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonBulkSearchVolumeLiveRequestInfo>, DataforseoLabsAmazonBulkSearchVolumeLiveResponseInfo>("/v3/dataforseo_labs/amazon/bulk_search_volume/live", body).await
}

pub async fn amazon_related_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonRelatedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsAmazonRelatedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonRelatedKeywordsLiveRequestInfo>, DataforseoLabsAmazonRelatedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/amazon/related_keywords/live", body).await
}

pub async fn amazon_ranked_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonRankedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsAmazonRankedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonRankedKeywordsLiveRequestInfo>, DataforseoLabsAmazonRankedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/amazon/ranked_keywords/live", body).await
}

pub async fn amazon_product_rank_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonProductRankOverviewLiveRequestInfo>) -> Result<DataforseoLabsAmazonProductRankOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonProductRankOverviewLiveRequestInfo>, DataforseoLabsAmazonProductRankOverviewLiveResponseInfo>("/v3/dataforseo_labs/amazon/product_rank_overview/live", body).await
}

pub async fn amazon_product_competitors_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonProductCompetitorsLiveRequestInfo>) -> Result<DataforseoLabsAmazonProductCompetitorsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonProductCompetitorsLiveRequestInfo>, DataforseoLabsAmazonProductCompetitorsLiveResponseInfo>("/v3/dataforseo_labs/amazon/product_competitors/live", body).await
}

pub async fn amazon_product_keyword_intersections_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAmazonProductKeywordIntersectionsLiveRequestInfo>) -> Result<DataforseoLabsAmazonProductKeywordIntersectionsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAmazonProductKeywordIntersectionsLiveRequestInfo>, DataforseoLabsAmazonProductKeywordIntersectionsLiveResponseInfo>("/v3/dataforseo_labs/amazon/product_keyword_intersections/live", body).await
}

pub async fn bing_bulk_keyword_difficulty_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingBulkKeywordDifficultyLiveRequestInfo>) -> Result<DataforseoLabsBingBulkKeywordDifficultyLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingBulkKeywordDifficultyLiveRequestInfo>, DataforseoLabsBingBulkKeywordDifficultyLiveResponseInfo>("/v3/dataforseo_labs/bing/bulk_keyword_difficulty/live", body).await
}

pub async fn bing_bulk_traffic_estimation_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingBulkTrafficEstimationLiveRequestInfo>) -> Result<DataforseoLabsBingBulkTrafficEstimationLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingBulkTrafficEstimationLiveRequestInfo>, DataforseoLabsBingBulkTrafficEstimationLiveResponseInfo>("/v3/dataforseo_labs/bing/bulk_traffic_estimation/live", body).await
}

pub async fn bing_competitors_domain_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingCompetitorsDomainLiveRequestInfo>) -> Result<DataforseoLabsBingCompetitorsDomainLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingCompetitorsDomainLiveRequestInfo>, DataforseoLabsBingCompetitorsDomainLiveResponseInfo>("/v3/dataforseo_labs/bing/competitors_domain/live", body).await
}

pub async fn bing_domain_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingDomainIntersectionLiveRequestInfo>) -> Result<DataforseoLabsBingDomainIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingDomainIntersectionLiveRequestInfo>, DataforseoLabsBingDomainIntersectionLiveResponseInfo>("/v3/dataforseo_labs/bing/domain_intersection/live", body).await
}

pub async fn bing_domain_rank_overview_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingDomainRankOverviewLiveRequestInfo>) -> Result<DataforseoLabsBingDomainRankOverviewLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingDomainRankOverviewLiveRequestInfo>, DataforseoLabsBingDomainRankOverviewLiveResponseInfo>("/v3/dataforseo_labs/bing/domain_rank_overview/live", body).await
}

pub async fn bing_page_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingPageIntersectionLiveRequestInfo>) -> Result<DataforseoLabsBingPageIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingPageIntersectionLiveRequestInfo>, DataforseoLabsBingPageIntersectionLiveResponseInfo>("/v3/dataforseo_labs/bing/page_intersection/live", body).await
}

pub async fn bing_ranked_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingRankedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsBingRankedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingRankedKeywordsLiveRequestInfo>, DataforseoLabsBingRankedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/bing/ranked_keywords/live", body).await
}

pub async fn bing_related_keywords_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingRelatedKeywordsLiveRequestInfo>) -> Result<DataforseoLabsBingRelatedKeywordsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingRelatedKeywordsLiveRequestInfo>, DataforseoLabsBingRelatedKeywordsLiveResponseInfo>("/v3/dataforseo_labs/bing/related_keywords/live", body).await
}

pub async fn bing_relevant_pages_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingRelevantPagesLiveRequestInfo>) -> Result<DataforseoLabsBingRelevantPagesLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingRelevantPagesLiveRequestInfo>, DataforseoLabsBingRelevantPagesLiveResponseInfo>("/v3/dataforseo_labs/bing/relevant_pages/live", body).await
}

pub async fn bing_serp_competitors_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingSerpCompetitorsLiveRequestInfo>) -> Result<DataforseoLabsBingSerpCompetitorsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingSerpCompetitorsLiveRequestInfo>, DataforseoLabsBingSerpCompetitorsLiveResponseInfo>("/v3/dataforseo_labs/bing/serp_competitors/live", body).await
}

pub async fn bing_subdomains_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsBingSubdomainsLiveRequestInfo>) -> Result<DataforseoLabsBingSubdomainsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsBingSubdomainsLiveRequestInfo>, DataforseoLabsBingSubdomainsLiveResponseInfo>("/v3/dataforseo_labs/bing/subdomains/live", body).await
}

pub async fn google_bulk_app_metrics_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleBulkAppMetricsLiveRequestInfo>) -> Result<DataforseoLabsGoogleBulkAppMetricsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleBulkAppMetricsLiveRequestInfo>, DataforseoLabsGoogleBulkAppMetricsLiveResponseInfo>("/v3/dataforseo_labs/google/bulk_app_metrics/live", body).await
}

pub async fn google_keywords_for_app_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleKeywordsForAppLiveRequestInfo>) -> Result<DataforseoLabsGoogleKeywordsForAppLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleKeywordsForAppLiveRequestInfo>, DataforseoLabsGoogleKeywordsForAppLiveResponseInfo>("/v3/dataforseo_labs/google/keywords_for_app/live", body).await
}

pub async fn google_app_competitors_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleAppCompetitorsLiveRequestInfo>) -> Result<DataforseoLabsGoogleAppCompetitorsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleAppCompetitorsLiveRequestInfo>, DataforseoLabsGoogleAppCompetitorsLiveResponseInfo>("/v3/dataforseo_labs/google/app_competitors/live", body).await
}

pub async fn google_app_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsGoogleAppIntersectionLiveRequestInfo>) -> Result<DataforseoLabsGoogleAppIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsGoogleAppIntersectionLiveRequestInfo>, DataforseoLabsGoogleAppIntersectionLiveResponseInfo>("/v3/dataforseo_labs/google/app_intersection/live", body).await
}

pub async fn apple_bulk_app_metrics_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAppleBulkAppMetricsLiveRequestInfo>) -> Result<DataforseoLabsAppleBulkAppMetricsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAppleBulkAppMetricsLiveRequestInfo>, DataforseoLabsAppleBulkAppMetricsLiveResponseInfo>("/v3/dataforseo_labs/apple/bulk_app_metrics/live", body).await
}

pub async fn apple_keywords_for_app_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAppleKeywordsForAppLiveRequestInfo>) -> Result<DataforseoLabsAppleKeywordsForAppLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAppleKeywordsForAppLiveRequestInfo>, DataforseoLabsAppleKeywordsForAppLiveResponseInfo>("/v3/dataforseo_labs/apple/keywords_for_app/live", body).await
}

pub async fn apple_app_competitors_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAppleAppCompetitorsLiveRequestInfo>) -> Result<DataforseoLabsAppleAppCompetitorsLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAppleAppCompetitorsLiveRequestInfo>, DataforseoLabsAppleAppCompetitorsLiveResponseInfo>("/v3/dataforseo_labs/apple/app_competitors/live", body).await
}

pub async fn apple_app_intersection_live(client: &DataForSeoClient, body: &Vec<DataforseoLabsAppleAppIntersectionLiveRequestInfo>) -> Result<DataforseoLabsAppleAppIntersectionLiveResponseInfo> {
    client.post::<Vec<DataforseoLabsAppleAppIntersectionLiveRequestInfo>, DataforseoLabsAppleAppIntersectionLiveResponseInfo>("/v3/dataforseo_labs/apple/app_intersection/live", body).await
}

pub async fn domain_analytics_id_list(client: &DataForSeoClient, body: &Vec<DomainAnalyticsIdListRequestInfo>) -> Result<DomainAnalyticsIdListResponseInfo> {
    client.post::<Vec<DomainAnalyticsIdListRequestInfo>, DomainAnalyticsIdListResponseInfo>("/v3/domain_analytics/id_list", body).await
}

pub async fn domain_analytics_errors(client: &DataForSeoClient, body: &Vec<DomainAnalyticsErrorsRequestInfo>) -> Result<DomainAnalyticsErrorsResponseInfo> {
    client.post::<Vec<DomainAnalyticsErrorsRequestInfo>, DomainAnalyticsErrorsResponseInfo>("/v3/domain_analytics/errors", body).await
}

pub async fn technologies_available_filters(client: &DataForSeoClient) -> Result<DomainAnalyticsTechnologiesAvailableFiltersResponseInfo> {
    let path = "/v3/domain_analytics/technologies/available_filters".to_string();
    client.get::<DomainAnalyticsTechnologiesAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn technologies_locations(client: &DataForSeoClient) -> Result<DomainAnalyticsTechnologiesLocationsResponseInfo> {
    let path = "/v3/domain_analytics/technologies/locations".to_string();
    client.get::<DomainAnalyticsTechnologiesLocationsResponseInfo>(
        &path
    ).await
}

pub async fn technologies_languages(client: &DataForSeoClient) -> Result<DomainAnalyticsTechnologiesLanguagesResponseInfo> {
    let path = "/v3/domain_analytics/technologies/languages".to_string();
    client.get::<DomainAnalyticsTechnologiesLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn technologies_technologies(client: &DataForSeoClient) -> Result<DomainAnalyticsTechnologiesTechnologiesResponseInfo> {
    let path = "/v3/domain_analytics/technologies/technologies".to_string();
    client.get::<DomainAnalyticsTechnologiesTechnologiesResponseInfo>(
        &path
    ).await
}

pub async fn technologies_aggregation_technologies_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesAggregationTechnologiesLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesAggregationTechnologiesLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesAggregationTechnologiesLiveRequestInfo>, DomainAnalyticsTechnologiesAggregationTechnologiesLiveResponseInfo>("/v3/domain_analytics/technologies/aggregation_technologies/live", body).await
}

pub async fn technologies_technologies_summary_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesTechnologiesSummaryLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesTechnologiesSummaryLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesTechnologiesSummaryLiveRequestInfo>, DomainAnalyticsTechnologiesTechnologiesSummaryLiveResponseInfo>("/v3/domain_analytics/technologies/technologies_summary/live", body).await
}

pub async fn technologies_technology_stats_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesTechnologyStatsLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesTechnologyStatsLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesTechnologyStatsLiveRequestInfo>, DomainAnalyticsTechnologiesTechnologyStatsLiveResponseInfo>("/v3/domain_analytics/technologies/technology_stats/live", body).await
}

pub async fn technologies_domains_by_technology_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesDomainsByTechnologyLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesDomainsByTechnologyLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesDomainsByTechnologyLiveRequestInfo>, DomainAnalyticsTechnologiesDomainsByTechnologyLiveResponseInfo>("/v3/domain_analytics/technologies/domains_by_technology/live", body).await
}

pub async fn technologies_domains_by_html_terms_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesDomainsByHtmlTermsLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesDomainsByHtmlTermsLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesDomainsByHtmlTermsLiveRequestInfo>, DomainAnalyticsTechnologiesDomainsByHtmlTermsLiveResponseInfo>("/v3/domain_analytics/technologies/domains_by_html_terms/live", body).await
}

pub async fn technologies_domain_technologies_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsTechnologiesDomainTechnologiesLiveRequestInfo>) -> Result<DomainAnalyticsTechnologiesDomainTechnologiesLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsTechnologiesDomainTechnologiesLiveRequestInfo>, DomainAnalyticsTechnologiesDomainTechnologiesLiveResponseInfo>("/v3/domain_analytics/technologies/domain_technologies/live", body).await
}

pub async fn whois_available_filters(client: &DataForSeoClient) -> Result<DomainAnalyticsWhoisAvailableFiltersResponseInfo> {
    let path = "/v3/domain_analytics/whois/available_filters".to_string();
    client.get::<DomainAnalyticsWhoisAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn whois_overview_live(client: &DataForSeoClient, body: &Vec<DomainAnalyticsWhoisOverviewLiveRequestInfo>) -> Result<DomainAnalyticsWhoisOverviewLiveResponseInfo> {
    client.post::<Vec<DomainAnalyticsWhoisOverviewLiveRequestInfo>, DomainAnalyticsWhoisOverviewLiveResponseInfo>("/v3/domain_analytics/whois/overview/live", body).await
}

pub async fn keywords_data_id_list(client: &DataForSeoClient, body: &Vec<KeywordsDataIdListRequestInfo>) -> Result<KeywordsDataIdListResponseInfo> {
    client.post::<Vec<KeywordsDataIdListRequestInfo>, KeywordsDataIdListResponseInfo>("/v3/keywords_data/id_list", body).await
}

pub async fn keywords_data_errors(client: &DataForSeoClient, body: &Vec<KeywordsDataErrorsRequestInfo>) -> Result<KeywordsDataErrorsResponseInfo> {
    client.post::<Vec<KeywordsDataErrorsRequestInfo>, KeywordsDataErrorsResponseInfo>("/v3/keywords_data/errors", body).await
}

pub async fn google_ads_status(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsStatusResponseInfo> {
    let path = "/v3/keywords_data/google_ads/status".to_string();
    client.get::<KeywordsDataGoogleAdsStatusResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_locations(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsLocationsResponseInfo> {
    let path = "/v3/keywords_data/google_ads/locations".to_string();
    client.get::<KeywordsDataGoogleAdsLocationsResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_locations_country(client: &DataForSeoClient, country: &str) -> Result<KeywordsDataGoogleAdsLocationsCountryResponseInfo> {
    let path = format!("/v3/keywords_data/google_ads/locations/{country}", country = country);
    client.get::<KeywordsDataGoogleAdsLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_languages(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsLanguagesResponseInfo> {
    let path = "/v3/keywords_data/google_ads/languages".to_string();
    client.get::<KeywordsDataGoogleAdsLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_volume_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsSearchVolumeTaskPostRequestInfo>) -> Result<KeywordsDataGoogleAdsSearchVolumeTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsSearchVolumeTaskPostRequestInfo>, KeywordsDataGoogleAdsSearchVolumeTaskPostResponseInfo>("/v3/keywords_data/google_ads/search_volume/task_post", body).await
}

pub async fn google_ads_search_volume_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsSearchVolumeTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/google_ads/search_volume/tasks_ready".to_string();
    client.get::<KeywordsDataGoogleAdsSearchVolumeTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_volume_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataGoogleAdsSearchVolumeTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/google_ads/search_volume/task_get/{id}", id = id);
    client.get::<KeywordsDataGoogleAdsSearchVolumeTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_search_volume_live(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsSearchVolumeLiveRequestInfo>) -> Result<KeywordsDataGoogleAdsSearchVolumeLiveResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsSearchVolumeLiveRequestInfo>, KeywordsDataGoogleAdsSearchVolumeLiveResponseInfo>("/v3/keywords_data/google_ads/search_volume/live", body).await
}

pub async fn google_ads_keywords_for_site_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsKeywordsForSiteTaskPostRequestInfo>) -> Result<KeywordsDataGoogleAdsKeywordsForSiteTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsKeywordsForSiteTaskPostRequestInfo>, KeywordsDataGoogleAdsKeywordsForSiteTaskPostResponseInfo>("/v3/keywords_data/google_ads/keywords_for_site/task_post", body).await
}

pub async fn google_ads_keywords_for_site_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsKeywordsForSiteTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/google_ads/keywords_for_site/tasks_ready".to_string();
    client.get::<KeywordsDataGoogleAdsKeywordsForSiteTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_keywords_for_site_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataGoogleAdsKeywordsForSiteTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/google_ads/keywords_for_site/task_get/{id}", id = id);
    client.get::<KeywordsDataGoogleAdsKeywordsForSiteTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_keywords_for_site_live(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsKeywordsForSiteLiveRequestInfo>) -> Result<KeywordsDataGoogleAdsKeywordsForSiteLiveResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsKeywordsForSiteLiveRequestInfo>, KeywordsDataGoogleAdsKeywordsForSiteLiveResponseInfo>("/v3/keywords_data/google_ads/keywords_for_site/live", body).await
}

pub async fn google_ads_keywords_for_keywords_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsKeywordsForKeywordsTaskPostRequestInfo>) -> Result<KeywordsDataGoogleAdsKeywordsForKeywordsTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsKeywordsForKeywordsTaskPostRequestInfo>, KeywordsDataGoogleAdsKeywordsForKeywordsTaskPostResponseInfo>("/v3/keywords_data/google_ads/keywords_for_keywords/task_post", body).await
}

pub async fn google_ads_keywords_for_keywords_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsKeywordsForKeywordsTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/google_ads/keywords_for_keywords/tasks_ready".to_string();
    client.get::<KeywordsDataGoogleAdsKeywordsForKeywordsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_keywords_for_keywords_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataGoogleAdsKeywordsForKeywordsTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/google_ads/keywords_for_keywords/task_get/{id}", id = id);
    client.get::<KeywordsDataGoogleAdsKeywordsForKeywordsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_keywords_for_keywords_live(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsKeywordsForKeywordsLiveRequestInfo>) -> Result<KeywordsDataGoogleAdsKeywordsForKeywordsLiveResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsKeywordsForKeywordsLiveRequestInfo>, KeywordsDataGoogleAdsKeywordsForKeywordsLiveResponseInfo>("/v3/keywords_data/google_ads/keywords_for_keywords/live", body).await
}

pub async fn google_ads_ad_traffic_by_keywords_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsAdTrafficByKeywordsTaskPostRequestInfo>) -> Result<KeywordsDataGoogleAdsAdTrafficByKeywordsTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsAdTrafficByKeywordsTaskPostRequestInfo>, KeywordsDataGoogleAdsAdTrafficByKeywordsTaskPostResponseInfo>("/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_post", body).await
}

pub async fn google_ads_ad_traffic_by_keywords_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataGoogleAdsAdTrafficByKeywordsTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/google_ads/ad_traffic_by_keywords/tasks_ready".to_string();
    client.get::<KeywordsDataGoogleAdsAdTrafficByKeywordsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_ad_traffic_by_keywords_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataGoogleAdsAdTrafficByKeywordsTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_get/{id}", id = id);
    client.get::<KeywordsDataGoogleAdsAdTrafficByKeywordsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_ads_ad_traffic_by_keywords_live(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleAdsAdTrafficByKeywordsLiveRequestInfo>) -> Result<KeywordsDataGoogleAdsAdTrafficByKeywordsLiveResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleAdsAdTrafficByKeywordsLiveRequestInfo>, KeywordsDataGoogleAdsAdTrafficByKeywordsLiveResponseInfo>("/v3/keywords_data/google_ads/ad_traffic_by_keywords/live", body).await
}

pub async fn google_trends_locations(client: &DataForSeoClient) -> Result<KeywordsDataGoogleTrendsLocationsResponseInfo> {
    let path = "/v3/keywords_data/google_trends/locations".to_string();
    client.get::<KeywordsDataGoogleTrendsLocationsResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_locations_country(client: &DataForSeoClient, country: &str) -> Result<KeywordsDataGoogleTrendsLocationsCountryResponseInfo> {
    let path = format!("/v3/keywords_data/google_trends/locations/{country}", country = country);
    client.get::<KeywordsDataGoogleTrendsLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_languages(client: &DataForSeoClient) -> Result<KeywordsDataGoogleTrendsLanguagesResponseInfo> {
    let path = "/v3/keywords_data/google_trends/languages".to_string();
    client.get::<KeywordsDataGoogleTrendsLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_categories(client: &DataForSeoClient) -> Result<KeywordsDataGoogleTrendsCategoriesResponseInfo> {
    let path = "/v3/keywords_data/google_trends/categories".to_string();
    client.get::<KeywordsDataGoogleTrendsCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_explore_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleTrendsExploreTaskPostRequestInfo>) -> Result<KeywordsDataGoogleTrendsExploreTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleTrendsExploreTaskPostRequestInfo>, KeywordsDataGoogleTrendsExploreTaskPostResponseInfo>("/v3/keywords_data/google_trends/explore/task_post", body).await
}

pub async fn google_trends_explore_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataGoogleTrendsExploreTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/google_trends/explore/tasks_ready".to_string();
    client.get::<KeywordsDataGoogleTrendsExploreTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_explore_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataGoogleTrendsExploreTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/google_trends/explore/task_get/{id}", id = id);
    client.get::<KeywordsDataGoogleTrendsExploreTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_trends_explore_live(client: &DataForSeoClient, body: &Vec<KeywordsDataGoogleTrendsExploreLiveRequestInfo>) -> Result<KeywordsDataGoogleTrendsExploreLiveResponseInfo> {
    client.post::<Vec<KeywordsDataGoogleTrendsExploreLiveRequestInfo>, KeywordsDataGoogleTrendsExploreLiveResponseInfo>("/v3/keywords_data/google_trends/explore/live", body).await
}

pub async fn dataforseo_trends_locations(client: &DataForSeoClient) -> Result<KeywordsDataDataforseoTrendsLocationsResponseInfo> {
    let path = "/v3/keywords_data/dataforseo_trends/locations".to_string();
    client.get::<KeywordsDataDataforseoTrendsLocationsResponseInfo>(
        &path
    ).await
}

pub async fn dataforseo_trends_locations_country(client: &DataForSeoClient, country: &str) -> Result<KeywordsDataDataforseoTrendsLocationsCountryResponseInfo> {
    let path = format!("/v3/keywords_data/dataforseo_trends/locations/{country}", country = country);
    client.get::<KeywordsDataDataforseoTrendsLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn dataforseo_trends_explore_live(client: &DataForSeoClient, body: &Vec<KeywordsDataDataforseoTrendsExploreLiveRequestInfo>) -> Result<KeywordsDataDataforseoTrendsExploreLiveResponseInfo> {
    client.post::<Vec<KeywordsDataDataforseoTrendsExploreLiveRequestInfo>, KeywordsDataDataforseoTrendsExploreLiveResponseInfo>("/v3/keywords_data/dataforseo_trends/explore/live", body).await
}

pub async fn dataforseo_trends_subregion_interests_live(client: &DataForSeoClient, body: &Vec<KeywordsDataDataforseoTrendsSubregionInterestsLiveRequestInfo>) -> Result<KeywordsDataDataforseoTrendsSubregionInterestsLiveResponseInfo> {
    client.post::<Vec<KeywordsDataDataforseoTrendsSubregionInterestsLiveRequestInfo>, KeywordsDataDataforseoTrendsSubregionInterestsLiveResponseInfo>("/v3/keywords_data/dataforseo_trends/subregion_interests/live", body).await
}

pub async fn dataforseo_trends_demography_live(client: &DataForSeoClient, body: &Vec<KeywordsDataDataforseoTrendsDemographyLiveRequestInfo>) -> Result<KeywordsDataDataforseoTrendsDemographyLiveResponseInfo> {
    client.post::<Vec<KeywordsDataDataforseoTrendsDemographyLiveRequestInfo>, KeywordsDataDataforseoTrendsDemographyLiveResponseInfo>("/v3/keywords_data/dataforseo_trends/demography/live", body).await
}

pub async fn dataforseo_trends_merged_data_live(client: &DataForSeoClient, body: &Vec<KeywordsDataDataforseoTrendsMergedDataLiveRequestInfo>) -> Result<KeywordsDataDataforseoTrendsMergedDataLiveResponseInfo> {
    client.post::<Vec<KeywordsDataDataforseoTrendsMergedDataLiveRequestInfo>, KeywordsDataDataforseoTrendsMergedDataLiveResponseInfo>("/v3/keywords_data/dataforseo_trends/merged_data/live", body).await
}

pub async fn keywords_data_bing_locations(client: &DataForSeoClient) -> Result<KeywordsDataBingLocationsResponseInfo> {
    let path = "/v3/keywords_data/bing/locations".to_string();
    client.get::<KeywordsDataBingLocationsResponseInfo>(
        &path
    ).await
}

pub async fn keywords_data_bing_languages(client: &DataForSeoClient) -> Result<KeywordsDataBingLanguagesResponseInfo> {
    let path = "/v3/keywords_data/bing/languages".to_string();
    client.get::<KeywordsDataBingLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingSearchVolumeTaskPostRequestInfo>) -> Result<KeywordsDataBingSearchVolumeTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingSearchVolumeTaskPostRequestInfo>, KeywordsDataBingSearchVolumeTaskPostResponseInfo>("/v3/keywords_data/bing/search_volume/task_post", body).await
}

pub async fn bing_search_volume_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingSearchVolumeTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/search_volume/tasks_ready".to_string();
    client.get::<KeywordsDataBingSearchVolumeTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingSearchVolumeTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/search_volume/task_get/{id}", id = id);
    client.get::<KeywordsDataBingSearchVolumeTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingSearchVolumeLiveRequestInfo>) -> Result<KeywordsDataBingSearchVolumeLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingSearchVolumeLiveRequestInfo>, KeywordsDataBingSearchVolumeLiveResponseInfo>("/v3/keywords_data/bing/search_volume/live", body).await
}

pub async fn bing_audience_estimation_job_functions(client: &DataForSeoClient) -> Result<KeywordsDataBingAudienceEstimationJobFunctionsResponseInfo> {
    let path = "/v3/keywords_data/bing/audience_estimation/job_functions".to_string();
    client.get::<KeywordsDataBingAudienceEstimationJobFunctionsResponseInfo>(
        &path
    ).await
}

pub async fn bing_audience_estimation_industries(client: &DataForSeoClient) -> Result<KeywordsDataBingAudienceEstimationIndustriesResponseInfo> {
    let path = "/v3/keywords_data/bing/audience_estimation/industries".to_string();
    client.get::<KeywordsDataBingAudienceEstimationIndustriesResponseInfo>(
        &path
    ).await
}

pub async fn bing_audience_estimation_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingAudienceEstimationTaskPostRequestInfo>) -> Result<KeywordsDataBingAudienceEstimationTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingAudienceEstimationTaskPostRequestInfo>, KeywordsDataBingAudienceEstimationTaskPostResponseInfo>("/v3/keywords_data/bing/audience_estimation/task_post", body).await
}

pub async fn bing_audience_estimation_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingAudienceEstimationTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/audience_estimation/tasks_ready".to_string();
    client.get::<KeywordsDataBingAudienceEstimationTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_audience_estimation_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingAudienceEstimationTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/audience_estimation/task_get/{id}", id = id);
    client.get::<KeywordsDataBingAudienceEstimationTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_audience_estimation_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingAudienceEstimationLiveRequestInfo>) -> Result<KeywordsDataBingAudienceEstimationLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingAudienceEstimationLiveRequestInfo>, KeywordsDataBingAudienceEstimationLiveResponseInfo>("/v3/keywords_data/bing/audience_estimation/live", body).await
}

pub async fn bing_keywords_for_site_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordsForSiteTaskPostRequestInfo>) -> Result<KeywordsDataBingKeywordsForSiteTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordsForSiteTaskPostRequestInfo>, KeywordsDataBingKeywordsForSiteTaskPostResponseInfo>("/v3/keywords_data/bing/keywords_for_site/task_post", body).await
}

pub async fn bing_keywords_for_site_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingKeywordsForSiteTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/keywords_for_site/tasks_ready".to_string();
    client.get::<KeywordsDataBingKeywordsForSiteTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_keywords_for_site_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingKeywordsForSiteTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/keywords_for_site/task_get/{id}", id = id);
    client.get::<KeywordsDataBingKeywordsForSiteTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_keywords_for_site_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordsForSiteLiveRequestInfo>) -> Result<KeywordsDataBingKeywordsForSiteLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordsForSiteLiveRequestInfo>, KeywordsDataBingKeywordsForSiteLiveResponseInfo>("/v3/keywords_data/bing/keywords_for_site/live", body).await
}

pub async fn bing_keywords_for_keywords_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordsForKeywordsTaskPostRequestInfo>) -> Result<KeywordsDataBingKeywordsForKeywordsTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordsForKeywordsTaskPostRequestInfo>, KeywordsDataBingKeywordsForKeywordsTaskPostResponseInfo>("/v3/keywords_data/bing/keywords_for_keywords/task_post", body).await
}

pub async fn bing_keywords_for_keywords_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingKeywordsForKeywordsTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/keywords_for_keywords/tasks_ready".to_string();
    client.get::<KeywordsDataBingKeywordsForKeywordsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_keywords_for_keywords_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingKeywordsForKeywordsTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/keywords_for_keywords/task_get/{id}", id = id);
    client.get::<KeywordsDataBingKeywordsForKeywordsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_keywords_for_keywords_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordsForKeywordsLiveRequestInfo>) -> Result<KeywordsDataBingKeywordsForKeywordsLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordsForKeywordsLiveRequestInfo>, KeywordsDataBingKeywordsForKeywordsLiveResponseInfo>("/v3/keywords_data/bing/keywords_for_keywords/live", body).await
}

pub async fn bing_keyword_performance_locations_and_languages(client: &DataForSeoClient) -> Result<KeywordsDataBingKeywordPerformanceLocationsAndLanguagesResponseInfo> {
    let path = "/v3/keywords_data/bing/keyword_performance/locations_and_languages".to_string();
    client.get::<KeywordsDataBingKeywordPerformanceLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn bing_keyword_performance_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordPerformanceTaskPostRequestInfo>) -> Result<KeywordsDataBingKeywordPerformanceTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordPerformanceTaskPostRequestInfo>, KeywordsDataBingKeywordPerformanceTaskPostResponseInfo>("/v3/keywords_data/bing/keyword_performance/task_post", body).await
}

pub async fn bing_keyword_performance_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingKeywordPerformanceTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/keyword_performance/tasks_ready".to_string();
    client.get::<KeywordsDataBingKeywordPerformanceTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_keyword_performance_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingKeywordPerformanceTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/keyword_performance/task_get/{id}", id = id);
    client.get::<KeywordsDataBingKeywordPerformanceTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_keyword_performance_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingKeywordPerformanceLiveRequestInfo>) -> Result<KeywordsDataBingKeywordPerformanceLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingKeywordPerformanceLiveRequestInfo>, KeywordsDataBingKeywordPerformanceLiveResponseInfo>("/v3/keywords_data/bing/keyword_performance/live", body).await
}

pub async fn bing_search_volume_history_locations_and_languages(client: &DataForSeoClient) -> Result<KeywordsDataBingSearchVolumeHistoryLocationsAndLanguagesResponseInfo> {
    let path = "/v3/keywords_data/bing/search_volume_history/locations_and_languages".to_string();
    client.get::<KeywordsDataBingSearchVolumeHistoryLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_history_task_post(client: &DataForSeoClient, body: &Vec<KeywordsDataBingSearchVolumeHistoryTaskPostRequestInfo>) -> Result<KeywordsDataBingSearchVolumeHistoryTaskPostResponseInfo> {
    client.post::<Vec<KeywordsDataBingSearchVolumeHistoryTaskPostRequestInfo>, KeywordsDataBingSearchVolumeHistoryTaskPostResponseInfo>("/v3/keywords_data/bing/search_volume_history/task_post", body).await
}

pub async fn bing_search_volume_history_tasks_ready(client: &DataForSeoClient) -> Result<KeywordsDataBingSearchVolumeHistoryTasksReadyResponseInfo> {
    let path = "/v3/keywords_data/bing/search_volume_history/tasks_ready".to_string();
    client.get::<KeywordsDataBingSearchVolumeHistoryTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_history_task_get(client: &DataForSeoClient, id: &str) -> Result<KeywordsDataBingSearchVolumeHistoryTaskGetResponseInfo> {
    let path = format!("/v3/keywords_data/bing/search_volume_history/task_get/{id}", id = id);
    client.get::<KeywordsDataBingSearchVolumeHistoryTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn bing_search_volume_history_live(client: &DataForSeoClient, body: &Vec<KeywordsDataBingSearchVolumeHistoryLiveRequestInfo>) -> Result<KeywordsDataBingSearchVolumeHistoryLiveResponseInfo> {
    client.post::<Vec<KeywordsDataBingSearchVolumeHistoryLiveRequestInfo>, KeywordsDataBingSearchVolumeHistoryLiveResponseInfo>("/v3/keywords_data/bing/search_volume_history/live", body).await
}

pub async fn clickstream_data_locations_and_languages(client: &DataForSeoClient) -> Result<KeywordsDataClickstreamDataLocationsAndLanguagesResponseInfo> {
    let path = "/v3/keywords_data/clickstream_data/locations_and_languages".to_string();
    client.get::<KeywordsDataClickstreamDataLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn clickstream_data_dataforseo_search_volume_live(client: &DataForSeoClient, body: &Vec<KeywordsDataClickstreamDataDataforseoSearchVolumeLiveRequestInfo>) -> Result<KeywordsDataClickstreamDataDataforseoSearchVolumeLiveResponseInfo> {
    client.post::<Vec<KeywordsDataClickstreamDataDataforseoSearchVolumeLiveRequestInfo>, KeywordsDataClickstreamDataDataforseoSearchVolumeLiveResponseInfo>("/v3/keywords_data/clickstream_data/dataforseo_search_volume/live", body).await
}

pub async fn clickstream_data_global_search_volume_live(client: &DataForSeoClient, body: &Vec<KeywordsDataClickstreamDataGlobalSearchVolumeLiveRequestInfo>) -> Result<KeywordsDataClickstreamDataGlobalSearchVolumeLiveResponseInfo> {
    client.post::<Vec<KeywordsDataClickstreamDataGlobalSearchVolumeLiveRequestInfo>, KeywordsDataClickstreamDataGlobalSearchVolumeLiveResponseInfo>("/v3/keywords_data/clickstream_data/global_search_volume/live", body).await
}

pub async fn clickstream_data_bulk_search_volume_live(client: &DataForSeoClient, body: &Vec<KeywordsDataClickstreamDataBulkSearchVolumeLiveRequestInfo>) -> Result<KeywordsDataClickstreamDataBulkSearchVolumeLiveResponseInfo> {
    client.post::<Vec<KeywordsDataClickstreamDataBulkSearchVolumeLiveRequestInfo>, KeywordsDataClickstreamDataBulkSearchVolumeLiveResponseInfo>("/v3/keywords_data/clickstream_data/bulk_search_volume/live", body).await
}

pub async fn backlinks_id_list(client: &DataForSeoClient, body: &Vec<BacklinksIdListRequestInfo>) -> Result<BacklinksIdListResponseInfo> {
    client.post::<Vec<BacklinksIdListRequestInfo>, BacklinksIdListResponseInfo>("/v3/backlinks/id_list", body).await
}

pub async fn backlinks_errors(client: &DataForSeoClient, body: &Vec<BacklinksErrorsRequestInfo>) -> Result<BacklinksErrorsResponseInfo> {
    client.post::<Vec<BacklinksErrorsRequestInfo>, BacklinksErrorsResponseInfo>("/v3/backlinks/errors", body).await
}

pub async fn backlinks_available_filters(client: &DataForSeoClient) -> Result<BacklinksAvailableFiltersResponseInfo> {
    let path = "/v3/backlinks/available_filters".to_string();
    client.get::<BacklinksAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn index(client: &DataForSeoClient) -> Result<BacklinksIndexResponseInfo> {
    let path = "/v3/backlinks/index".to_string();
    client.get::<BacklinksIndexResponseInfo>(
        &path
    ).await
}

pub async fn summary_live(client: &DataForSeoClient, body: &Vec<BacklinksSummaryLiveRequestInfo>) -> Result<BacklinksSummaryLiveResponseInfo> {
    client.post::<Vec<BacklinksSummaryLiveRequestInfo>, BacklinksSummaryLiveResponseInfo>("/v3/backlinks/summary/live", body).await
}

pub async fn history_live(client: &DataForSeoClient, body: &Vec<BacklinksHistoryLiveRequestInfo>) -> Result<BacklinksHistoryLiveResponseInfo> {
    client.post::<Vec<BacklinksHistoryLiveRequestInfo>, BacklinksHistoryLiveResponseInfo>("/v3/backlinks/history/live", body).await
}

pub async fn backlinks_live(client: &DataForSeoClient, body: &Vec<BacklinksBacklinksLiveRequestInfo>) -> Result<BacklinksBacklinksLiveResponseInfo> {
    client.post::<Vec<BacklinksBacklinksLiveRequestInfo>, BacklinksBacklinksLiveResponseInfo>("/v3/backlinks/backlinks/live", body).await
}

pub async fn anchors_live(client: &DataForSeoClient, body: &Vec<BacklinksAnchorsLiveRequestInfo>) -> Result<BacklinksAnchorsLiveResponseInfo> {
    client.post::<Vec<BacklinksAnchorsLiveRequestInfo>, BacklinksAnchorsLiveResponseInfo>("/v3/backlinks/anchors/live", body).await
}

pub async fn domain_pages_live(client: &DataForSeoClient, body: &Vec<BacklinksDomainPagesLiveRequestInfo>) -> Result<BacklinksDomainPagesLiveResponseInfo> {
    client.post::<Vec<BacklinksDomainPagesLiveRequestInfo>, BacklinksDomainPagesLiveResponseInfo>("/v3/backlinks/domain_pages/live", body).await
}

pub async fn domain_pages_summary_live(client: &DataForSeoClient, body: &Vec<BacklinksDomainPagesSummaryLiveRequestInfo>) -> Result<BacklinksDomainPagesSummaryLiveResponseInfo> {
    client.post::<Vec<BacklinksDomainPagesSummaryLiveRequestInfo>, BacklinksDomainPagesSummaryLiveResponseInfo>("/v3/backlinks/domain_pages_summary/live", body).await
}

pub async fn referring_domains_live(client: &DataForSeoClient, body: &Vec<BacklinksReferringDomainsLiveRequestInfo>) -> Result<BacklinksReferringDomainsLiveResponseInfo> {
    client.post::<Vec<BacklinksReferringDomainsLiveRequestInfo>, BacklinksReferringDomainsLiveResponseInfo>("/v3/backlinks/referring_domains/live", body).await
}

pub async fn referring_networks_live(client: &DataForSeoClient, body: &Vec<BacklinksReferringNetworksLiveRequestInfo>) -> Result<BacklinksReferringNetworksLiveResponseInfo> {
    client.post::<Vec<BacklinksReferringNetworksLiveRequestInfo>, BacklinksReferringNetworksLiveResponseInfo>("/v3/backlinks/referring_networks/live", body).await
}

pub async fn competitors_live(client: &DataForSeoClient, body: &Vec<BacklinksCompetitorsLiveRequestInfo>) -> Result<BacklinksCompetitorsLiveResponseInfo> {
    client.post::<Vec<BacklinksCompetitorsLiveRequestInfo>, BacklinksCompetitorsLiveResponseInfo>("/v3/backlinks/competitors/live", body).await
}

pub async fn domain_intersection_live(client: &DataForSeoClient, body: &Vec<BacklinksDomainIntersectionLiveRequestInfo>) -> Result<BacklinksDomainIntersectionLiveResponseInfo> {
    client.post::<Vec<BacklinksDomainIntersectionLiveRequestInfo>, BacklinksDomainIntersectionLiveResponseInfo>("/v3/backlinks/domain_intersection/live", body).await
}

pub async fn page_intersection_live(client: &DataForSeoClient, body: &Vec<BacklinksPageIntersectionLiveRequestInfo>) -> Result<BacklinksPageIntersectionLiveResponseInfo> {
    client.post::<Vec<BacklinksPageIntersectionLiveRequestInfo>, BacklinksPageIntersectionLiveResponseInfo>("/v3/backlinks/page_intersection/live", body).await
}

pub async fn timeseries_summary_live(client: &DataForSeoClient, body: &Vec<BacklinksTimeseriesSummaryLiveRequestInfo>) -> Result<BacklinksTimeseriesSummaryLiveResponseInfo> {
    client.post::<Vec<BacklinksTimeseriesSummaryLiveRequestInfo>, BacklinksTimeseriesSummaryLiveResponseInfo>("/v3/backlinks/timeseries_summary/live", body).await
}

pub async fn timeseries_new_lost_summary_live(client: &DataForSeoClient, body: &Vec<BacklinksTimeseriesNewLostSummaryLiveRequestInfo>) -> Result<BacklinksTimeseriesNewLostSummaryLiveResponseInfo> {
    client.post::<Vec<BacklinksTimeseriesNewLostSummaryLiveRequestInfo>, BacklinksTimeseriesNewLostSummaryLiveResponseInfo>("/v3/backlinks/timeseries_new_lost_summary/live", body).await
}

pub async fn bulk_ranks_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkRanksLiveRequestInfo>) -> Result<BacklinksBulkRanksLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkRanksLiveRequestInfo>, BacklinksBulkRanksLiveResponseInfo>("/v3/backlinks/bulk_ranks/live", body).await
}

pub async fn bulk_backlinks_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkBacklinksLiveRequestInfo>) -> Result<BacklinksBulkBacklinksLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkBacklinksLiveRequestInfo>, BacklinksBulkBacklinksLiveResponseInfo>("/v3/backlinks/bulk_backlinks/live", body).await
}

pub async fn bulk_spam_score_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkSpamScoreLiveRequestInfo>) -> Result<BacklinksBulkSpamScoreLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkSpamScoreLiveRequestInfo>, BacklinksBulkSpamScoreLiveResponseInfo>("/v3/backlinks/bulk_spam_score/live", body).await
}

pub async fn bulk_referring_domains_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkReferringDomainsLiveRequestInfo>) -> Result<BacklinksBulkReferringDomainsLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkReferringDomainsLiveRequestInfo>, BacklinksBulkReferringDomainsLiveResponseInfo>("/v3/backlinks/bulk_referring_domains/live", body).await
}

pub async fn bulk_new_lost_backlinks_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkNewLostBacklinksLiveRequestInfo>) -> Result<BacklinksBulkNewLostBacklinksLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkNewLostBacklinksLiveRequestInfo>, BacklinksBulkNewLostBacklinksLiveResponseInfo>("/v3/backlinks/bulk_new_lost_backlinks/live", body).await
}

pub async fn bulk_new_lost_referring_domains_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkNewLostReferringDomainsLiveRequestInfo>) -> Result<BacklinksBulkNewLostReferringDomainsLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkNewLostReferringDomainsLiveRequestInfo>, BacklinksBulkNewLostReferringDomainsLiveResponseInfo>("/v3/backlinks/bulk_new_lost_referring_domains/live", body).await
}

pub async fn bulk_pages_summary_live(client: &DataForSeoClient, body: &Vec<BacklinksBulkPagesSummaryLiveRequestInfo>) -> Result<BacklinksBulkPagesSummaryLiveResponseInfo> {
    client.post::<Vec<BacklinksBulkPagesSummaryLiveRequestInfo>, BacklinksBulkPagesSummaryLiveResponseInfo>("/v3/backlinks/bulk_pages_summary/live", body).await
}

pub async fn chat_gpt_llm_scraper_locations(client: &DataForSeoClient) -> Result<AiOptimizationChatGptLlmScraperLocationsResponseInfo> {
    let path = "/v3/ai_optimization/chat_gpt/llm_scraper/locations".to_string();
    client.get::<AiOptimizationChatGptLlmScraperLocationsResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_scraper_locations_country(client: &DataForSeoClient, country: &str) -> Result<AiOptimizationChatGptLlmScraperLocationsCountryResponseInfo> {
    let path = format!("/v3/ai_optimization/chat_gpt/llm_scraper/locations/{country}", country = country);
    client.get::<AiOptimizationChatGptLlmScraperLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_scraper_languages(client: &DataForSeoClient) -> Result<AiOptimizationChatGptLlmScraperLanguagesResponseInfo> {
    let path = "/v3/ai_optimization/chat_gpt/llm_scraper/languages".to_string();
    client.get::<AiOptimizationChatGptLlmScraperLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_scraper_task_post(client: &DataForSeoClient, body: &Vec<AiOptimizationChatGptLlmScraperTaskPostRequestInfo>) -> Result<AiOptimizationChatGptLlmScraperTaskPostResponseInfo> {
    client.post::<Vec<AiOptimizationChatGptLlmScraperTaskPostRequestInfo>, AiOptimizationChatGptLlmScraperTaskPostResponseInfo>("/v3/ai_optimization/chat_gpt/llm_scraper/task_post", body).await
}

pub async fn chat_gpt_llm_scraper_tasks_ready(client: &DataForSeoClient) -> Result<AiOptimizationChatGptLlmScraperTasksReadyResponseInfo> {
    let path = "/v3/ai_optimization/chat_gpt/llm_scraper/tasks_ready".to_string();
    client.get::<AiOptimizationChatGptLlmScraperTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_scraper_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AiOptimizationChatGptLlmScraperTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/ai_optimization/chat_gpt/llm_scraper/task_get/advanced/{id}", id = id);
    client.get::<AiOptimizationChatGptLlmScraperTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_scraper_task_get_html(client: &DataForSeoClient, id: &str) -> Result<AiOptimizationChatGptLlmScraperTaskGetHtmlResponseInfo> {
    let path = format!("/v3/ai_optimization/chat_gpt/llm_scraper/task_get/html/{id}", id = id);
    client.get::<AiOptimizationChatGptLlmScraperTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn llm_mentions_locations_and_languages(client: &DataForSeoClient) -> Result<AiOptimizationLlmMentionsLocationsAndLanguagesResponseInfo> {
    let path = "/v3/ai_optimization/llm_mentions/locations_and_languages".to_string();
    client.get::<AiOptimizationLlmMentionsLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn llm_mentions_available_filters(client: &DataForSeoClient) -> Result<AiOptimizationLlmMentionsAvailableFiltersResponseInfo> {
    let path = "/v3/ai_optimization/llm_mentions/available_filters".to_string();
    client.get::<AiOptimizationLlmMentionsAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn llm_mentions_search_live(client: &DataForSeoClient, body: &Vec<AiOptimizationLlmMentionsSearchLiveRequestInfo>) -> Result<AiOptimizationLlmMentionsSearchLiveResponseInfo> {
    client.post::<Vec<AiOptimizationLlmMentionsSearchLiveRequestInfo>, AiOptimizationLlmMentionsSearchLiveResponseInfo>("/v3/ai_optimization/llm_mentions/search/live", body).await
}

pub async fn llm_mentions_top_pages_live(client: &DataForSeoClient, body: &Vec<AiOptimizationLlmMentionsTopPagesLiveRequestInfo>) -> Result<AiOptimizationLlmMentionsTopPagesLiveResponseInfo> {
    client.post::<Vec<AiOptimizationLlmMentionsTopPagesLiveRequestInfo>, AiOptimizationLlmMentionsTopPagesLiveResponseInfo>("/v3/ai_optimization/llm_mentions/top_pages/live", body).await
}

pub async fn llm_mentions_top_domains_live(client: &DataForSeoClient, body: &Vec<AiOptimizationLlmMentionsTopDomainsLiveRequestInfo>) -> Result<AiOptimizationLlmMentionsTopDomainsLiveResponseInfo> {
    client.post::<Vec<AiOptimizationLlmMentionsTopDomainsLiveRequestInfo>, AiOptimizationLlmMentionsTopDomainsLiveResponseInfo>("/v3/ai_optimization/llm_mentions/top_domains/live", body).await
}

pub async fn llm_mentions_aggregated_metrics_live(client: &DataForSeoClient, body: &Vec<AiOptimizationLlmMentionsAggregatedMetricsLiveRequestInfo>) -> Result<AiOptimizationLlmMentionsAggregatedMetricsLiveResponseInfo> {
    client.post::<Vec<AiOptimizationLlmMentionsAggregatedMetricsLiveRequestInfo>, AiOptimizationLlmMentionsAggregatedMetricsLiveResponseInfo>("/v3/ai_optimization/llm_mentions/aggregated_metrics/live", body).await
}

pub async fn llm_mentions_cross_aggregated_metrics_live(client: &DataForSeoClient, body: &Vec<AiOptimizationLlmMentionsCrossAggregatedMetricsLiveRequestInfo>) -> Result<AiOptimizationLlmMentionsCrossAggregatedMetricsLiveResponseInfo> {
    client.post::<Vec<AiOptimizationLlmMentionsCrossAggregatedMetricsLiveRequestInfo>, AiOptimizationLlmMentionsCrossAggregatedMetricsLiveResponseInfo>("/v3/ai_optimization/llm_mentions/cross_aggregated_metrics/live", body).await
}

pub async fn chat_gpt_llm_responses_models(client: &DataForSeoClient) -> Result<AiOptimizationChatGptLlmResponsesModelsResponseInfo> {
    let path = "/v3/ai_optimization/chat_gpt/llm_responses/models".to_string();
    client.get::<AiOptimizationChatGptLlmResponsesModelsResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_responses_live(client: &DataForSeoClient, body: &Vec<AiOptimizationChatGptLlmResponsesLiveRequestInfo>) -> Result<AiOptimizationChatGptLlmResponsesLiveResponseInfo> {
    client.post::<Vec<AiOptimizationChatGptLlmResponsesLiveRequestInfo>, AiOptimizationChatGptLlmResponsesLiveResponseInfo>("/v3/ai_optimization/chat_gpt/llm_responses/live", body).await
}

pub async fn chat_gpt_llm_responses_task_post(client: &DataForSeoClient, body: &Vec<AiOptimizationChatGptLlmResponsesTaskPostRequestInfo>) -> Result<AiOptimizationChatGptLlmResponsesTaskPostResponseInfo> {
    client.post::<Vec<AiOptimizationChatGptLlmResponsesTaskPostRequestInfo>, AiOptimizationChatGptLlmResponsesTaskPostResponseInfo>("/v3/ai_optimization/chat_gpt/llm_responses/task_post", body).await
}

pub async fn chat_gpt_llm_responses_tasks_ready(client: &DataForSeoClient) -> Result<AiOptimizationChatGptLlmResponsesTasksReadyResponseInfo> {
    let path = "/v3/ai_optimization/chat_gpt/llm_responses/tasks_ready".to_string();
    client.get::<AiOptimizationChatGptLlmResponsesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn chat_gpt_llm_responses_task_get(client: &DataForSeoClient, id: &str) -> Result<AiOptimizationChatGptLlmResponsesTaskGetResponseInfo> {
    let path = format!("/v3/ai_optimization/chat_gpt/llm_responses/task_get/{id}", id = id);
    client.get::<AiOptimizationChatGptLlmResponsesTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn claude_llm_responses_models(client: &DataForSeoClient) -> Result<AiOptimizationClaudeLlmResponsesModelsResponseInfo> {
    let path = "/v3/ai_optimization/claude/llm_responses/models".to_string();
    client.get::<AiOptimizationClaudeLlmResponsesModelsResponseInfo>(
        &path
    ).await
}

pub async fn claude_llm_responses_live(client: &DataForSeoClient, body: &Vec<AiOptimizationClaudeLlmResponsesLiveRequestInfo>) -> Result<AiOptimizationClaudeLlmResponsesLiveResponseInfo> {
    client.post::<Vec<AiOptimizationClaudeLlmResponsesLiveRequestInfo>, AiOptimizationClaudeLlmResponsesLiveResponseInfo>("/v3/ai_optimization/claude/llm_responses/live", body).await
}

pub async fn claude_llm_responses_task_post(client: &DataForSeoClient, body: &Vec<AiOptimizationClaudeLlmResponsesTaskPostRequestInfo>) -> Result<AiOptimizationClaudeLlmResponsesTaskPostResponseInfo> {
    client.post::<Vec<AiOptimizationClaudeLlmResponsesTaskPostRequestInfo>, AiOptimizationClaudeLlmResponsesTaskPostResponseInfo>("/v3/ai_optimization/claude/llm_responses/task_post", body).await
}

pub async fn claude_llm_responses_tasks_ready(client: &DataForSeoClient) -> Result<AiOptimizationClaudeLlmResponsesTasksReadyResponseInfo> {
    let path = "/v3/ai_optimization/claude/llm_responses/tasks_ready".to_string();
    client.get::<AiOptimizationClaudeLlmResponsesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn claude_llm_responses_task_get(client: &DataForSeoClient, id: &str) -> Result<AiOptimizationClaudeLlmResponsesTaskGetResponseInfo> {
    let path = format!("/v3/ai_optimization/claude/llm_responses/task_get/{id}", id = id);
    client.get::<AiOptimizationClaudeLlmResponsesTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn gemini_llm_responses_models(client: &DataForSeoClient) -> Result<AiOptimizationGeminiLlmResponsesModelsResponseInfo> {
    let path = "/v3/ai_optimization/gemini/llm_responses/models".to_string();
    client.get::<AiOptimizationGeminiLlmResponsesModelsResponseInfo>(
        &path
    ).await
}

pub async fn gemini_llm_responses_task_post(client: &DataForSeoClient, body: &Vec<AiOptimizationGeminiLlmResponsesTaskPostRequestInfo>) -> Result<AiOptimizationGeminiLlmResponsesTaskPostResponseInfo> {
    client.post::<Vec<AiOptimizationGeminiLlmResponsesTaskPostRequestInfo>, AiOptimizationGeminiLlmResponsesTaskPostResponseInfo>("/v3/ai_optimization/gemini/llm_responses/task_post", body).await
}

pub async fn gemini_llm_responses_tasks_ready(client: &DataForSeoClient) -> Result<AiOptimizationGeminiLlmResponsesTasksReadyResponseInfo> {
    let path = "/v3/ai_optimization/gemini/llm_responses/tasks_ready".to_string();
    client.get::<AiOptimizationGeminiLlmResponsesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn gemini_llm_responses_task_get(client: &DataForSeoClient, id: &str) -> Result<AiOptimizationGeminiLlmResponsesTaskGetResponseInfo> {
    let path = format!("/v3/ai_optimization/gemini/llm_responses/task_get/{id}", id = id);
    client.get::<AiOptimizationGeminiLlmResponsesTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn gemini_llm_responses_live(client: &DataForSeoClient, body: &Vec<AiOptimizationGeminiLlmResponsesLiveRequestInfo>) -> Result<AiOptimizationGeminiLlmResponsesLiveResponseInfo> {
    client.post::<Vec<AiOptimizationGeminiLlmResponsesLiveRequestInfo>, AiOptimizationGeminiLlmResponsesLiveResponseInfo>("/v3/ai_optimization/gemini/llm_responses/live", body).await
}

pub async fn perplexity_llm_responses_models(client: &DataForSeoClient) -> Result<AiOptimizationPerplexityLlmResponsesModelsResponseInfo> {
    let path = "/v3/ai_optimization/perplexity/llm_responses/models".to_string();
    client.get::<AiOptimizationPerplexityLlmResponsesModelsResponseInfo>(
        &path
    ).await
}

pub async fn perplexity_llm_responses_live(client: &DataForSeoClient, body: &Vec<AiOptimizationPerplexityLlmResponsesLiveRequestInfo>) -> Result<AiOptimizationPerplexityLlmResponsesLiveResponseInfo> {
    client.post::<Vec<AiOptimizationPerplexityLlmResponsesLiveRequestInfo>, AiOptimizationPerplexityLlmResponsesLiveResponseInfo>("/v3/ai_optimization/perplexity/llm_responses/live", body).await
}

pub async fn ai_keyword_data_available_filters(client: &DataForSeoClient) -> Result<AiOptimizationAiKeywordDataAvailableFiltersResponseInfo> {
    let path = "/v3/ai_optimization/ai_keyword_data/available_filters".to_string();
    client.get::<AiOptimizationAiKeywordDataAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn ai_keyword_data_locations_and_languages(client: &DataForSeoClient) -> Result<AiOptimizationAiKeywordDataLocationsAndLanguagesResponseInfo> {
    let path = "/v3/ai_optimization/ai_keyword_data/locations_and_languages".to_string();
    client.get::<AiOptimizationAiKeywordDataLocationsAndLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn ai_keyword_data_keywords_search_volume_live(client: &DataForSeoClient, body: &Vec<AiOptimizationAiKeywordDataKeywordsSearchVolumeLiveRequestInfo>) -> Result<AiOptimizationAiKeywordDataKeywordsSearchVolumeLiveResponseInfo> {
    client.post::<Vec<AiOptimizationAiKeywordDataKeywordsSearchVolumeLiveRequestInfo>, AiOptimizationAiKeywordDataKeywordsSearchVolumeLiveResponseInfo>("/v3/ai_optimization/ai_keyword_data/keywords_search_volume/live", body).await
}

pub async fn on_page_id_list(client: &DataForSeoClient, body: &Vec<OnPageIdListRequestInfo>) -> Result<OnPageIdListResponseInfo> {
    client.post::<Vec<OnPageIdListRequestInfo>, OnPageIdListResponseInfo>("/v3/on_page/id_list", body).await
}

pub async fn on_page_errors(client: &DataForSeoClient, body: &Vec<OnPageErrorsRequestInfo>) -> Result<OnPageErrorsResponseInfo> {
    client.post::<Vec<OnPageErrorsRequestInfo>, OnPageErrorsResponseInfo>("/v3/on_page/errors", body).await
}

pub async fn force_stop(client: &DataForSeoClient, body: &Vec<OnPageForceStopRequestInfo>) -> Result<OnPageForceStopResponseInfo> {
    client.post::<Vec<OnPageForceStopRequestInfo>, OnPageForceStopResponseInfo>("/v3/on_page/force_stop", body).await
}

pub async fn on_page_available_filters(client: &DataForSeoClient) -> Result<OnPageAvailableFiltersResponseInfo> {
    let path = "/v3/on_page/available_filters".to_string();
    client.get::<OnPageAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn task_post(client: &DataForSeoClient, body: &Vec<OnPageTaskPostRequestInfo>) -> Result<OnPageTaskPostResponseInfo> {
    client.post::<Vec<OnPageTaskPostRequestInfo>, OnPageTaskPostResponseInfo>("/v3/on_page/task_post", body).await
}

pub async fn on_page_tasks_ready(client: &DataForSeoClient) -> Result<OnPageTasksReadyResponseInfo> {
    let path = "/v3/on_page/tasks_ready".to_string();
    client.get::<OnPageTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn summary(client: &DataForSeoClient, id: &str) -> Result<OnPageSummaryResponseInfo> {
    let path = format!("/v3/on_page/summary/{id}", id = id);
    client.get::<OnPageSummaryResponseInfo>(
        &path
    ).await
}

pub async fn pages(client: &DataForSeoClient, body: &Vec<OnPagePagesRequestInfo>) -> Result<OnPagePagesResponseInfo> {
    client.post::<Vec<OnPagePagesRequestInfo>, OnPagePagesResponseInfo>("/v3/on_page/pages", body).await
}

pub async fn pages_by_resource(client: &DataForSeoClient, body: &Vec<OnPagePagesByResourceRequestInfo>) -> Result<OnPagePagesByResourceResponseInfo> {
    client.post::<Vec<OnPagePagesByResourceRequestInfo>, OnPagePagesByResourceResponseInfo>("/v3/on_page/pages_by_resource", body).await
}

pub async fn resources(client: &DataForSeoClient, body: &Vec<OnPageResourcesRequestInfo>) -> Result<OnPageResourcesResponseInfo> {
    client.post::<Vec<OnPageResourcesRequestInfo>, OnPageResourcesResponseInfo>("/v3/on_page/resources", body).await
}

pub async fn duplicate_tags(client: &DataForSeoClient, body: &Vec<OnPageDuplicateTagsRequestInfo>) -> Result<OnPageDuplicateTagsResponseInfo> {
    client.post::<Vec<OnPageDuplicateTagsRequestInfo>, OnPageDuplicateTagsResponseInfo>("/v3/on_page/duplicate_tags", body).await
}

pub async fn duplicate_content(client: &DataForSeoClient, body: &Vec<OnPageDuplicateContentRequestInfo>) -> Result<OnPageDuplicateContentResponseInfo> {
    client.post::<Vec<OnPageDuplicateContentRequestInfo>, OnPageDuplicateContentResponseInfo>("/v3/on_page/duplicate_content", body).await
}

pub async fn links(client: &DataForSeoClient, body: &Vec<OnPageLinksRequestInfo>) -> Result<OnPageLinksResponseInfo> {
    client.post::<Vec<OnPageLinksRequestInfo>, OnPageLinksResponseInfo>("/v3/on_page/links", body).await
}

pub async fn redirect_chains(client: &DataForSeoClient, body: &Vec<OnPageRedirectChainsRequestInfo>) -> Result<OnPageRedirectChainsResponseInfo> {
    client.post::<Vec<OnPageRedirectChainsRequestInfo>, OnPageRedirectChainsResponseInfo>("/v3/on_page/redirect_chains", body).await
}

pub async fn non_indexable(client: &DataForSeoClient, body: &Vec<OnPageNonIndexableRequestInfo>) -> Result<OnPageNonIndexableResponseInfo> {
    client.post::<Vec<OnPageNonIndexableRequestInfo>, OnPageNonIndexableResponseInfo>("/v3/on_page/non_indexable", body).await
}

pub async fn waterfall(client: &DataForSeoClient, body: &Vec<OnPageWaterfallRequestInfo>) -> Result<OnPageWaterfallResponseInfo> {
    client.post::<Vec<OnPageWaterfallRequestInfo>, OnPageWaterfallResponseInfo>("/v3/on_page/waterfall", body).await
}

pub async fn keyword_density(client: &DataForSeoClient, body: &Vec<OnPageKeywordDensityRequestInfo>) -> Result<OnPageKeywordDensityResponseInfo> {
    client.post::<Vec<OnPageKeywordDensityRequestInfo>, OnPageKeywordDensityResponseInfo>("/v3/on_page/keyword_density", body).await
}

pub async fn microdata(client: &DataForSeoClient, body: &Vec<OnPageMicrodataRequestInfo>) -> Result<OnPageMicrodataResponseInfo> {
    client.post::<Vec<OnPageMicrodataRequestInfo>, OnPageMicrodataResponseInfo>("/v3/on_page/microdata", body).await
}

pub async fn raw_html(client: &DataForSeoClient, body: &Vec<OnPageRawHtmlRequestInfo>) -> Result<OnPageRawHtmlResponseInfo> {
    client.post::<Vec<OnPageRawHtmlRequestInfo>, OnPageRawHtmlResponseInfo>("/v3/on_page/raw_html", body).await
}

pub async fn page_screenshot(client: &DataForSeoClient, body: &Vec<OnPagePageScreenshotRequestInfo>) -> Result<OnPagePageScreenshotResponseInfo> {
    client.post::<Vec<OnPagePageScreenshotRequestInfo>, OnPagePageScreenshotResponseInfo>("/v3/on_page/page_screenshot", body).await
}

pub async fn content_parsing(client: &DataForSeoClient, body: &Vec<OnPageContentParsingRequestInfo>) -> Result<OnPageContentParsingResponseInfo> {
    client.post::<Vec<OnPageContentParsingRequestInfo>, OnPageContentParsingResponseInfo>("/v3/on_page/content_parsing", body).await
}

pub async fn content_parsing_live(client: &DataForSeoClient, body: &Vec<OnPageContentParsingLiveRequestInfo>) -> Result<OnPageContentParsingLiveResponseInfo> {
    client.post::<Vec<OnPageContentParsingLiveRequestInfo>, OnPageContentParsingLiveResponseInfo>("/v3/on_page/content_parsing/live", body).await
}

pub async fn instant_pages(client: &DataForSeoClient, body: &Vec<OnPageInstantPagesRequestInfo>) -> Result<OnPageInstantPagesResponseInfo> {
    client.post::<Vec<OnPageInstantPagesRequestInfo>, OnPageInstantPagesResponseInfo>("/v3/on_page/instant_pages", body).await
}

pub async fn lighthouse_languages(client: &DataForSeoClient) -> Result<OnPageLighthouseLanguagesResponseInfo> {
    let path = "/v3/on_page/lighthouse/languages".to_string();
    client.get::<OnPageLighthouseLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn lighthouse_audits(client: &DataForSeoClient) -> Result<OnPageLighthouseAuditsResponseInfo> {
    let path = "/v3/on_page/lighthouse/audits".to_string();
    client.get::<OnPageLighthouseAuditsResponseInfo>(
        &path
    ).await
}

pub async fn lighthouse_versions(client: &DataForSeoClient) -> Result<OnPageLighthouseVersionsResponseInfo> {
    let path = "/v3/on_page/lighthouse/versions".to_string();
    client.get::<OnPageLighthouseVersionsResponseInfo>(
        &path
    ).await
}

pub async fn lighthouse_task_post(client: &DataForSeoClient, body: &Vec<OnPageLighthouseTaskPostRequestInfo>) -> Result<OnPageLighthouseTaskPostResponseInfo> {
    client.post::<Vec<OnPageLighthouseTaskPostRequestInfo>, OnPageLighthouseTaskPostResponseInfo>("/v3/on_page/lighthouse/task_post", body).await
}

pub async fn lighthouse_tasks_ready(client: &DataForSeoClient) -> Result<OnPageLighthouseTasksReadyResponseInfo> {
    let path = "/v3/on_page/lighthouse/tasks_ready".to_string();
    client.get::<OnPageLighthouseTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn lighthouse_task_get_json(client: &DataForSeoClient, id: &str) -> Result<OnPageLighthouseTaskGetJsonResponseInfo> {
    let path = format!("/v3/on_page/lighthouse/task_get/json/{id}", id = id);
    client.get::<OnPageLighthouseTaskGetJsonResponseInfo>(
        &path
    ).await
}

pub async fn lighthouse_live_json(client: &DataForSeoClient, body: &Vec<OnPageLighthouseLiveJsonRequestInfo>) -> Result<OnPageLighthouseLiveJsonResponseInfo> {
    client.post::<Vec<OnPageLighthouseLiveJsonRequestInfo>, OnPageLighthouseLiveJsonResponseInfo>("/v3/on_page/lighthouse/live/json", body).await
}

pub async fn content_analysis_id_list(client: &DataForSeoClient, body: &Vec<ContentAnalysisIdListRequestInfo>) -> Result<ContentAnalysisIdListResponseInfo> {
    client.post::<Vec<ContentAnalysisIdListRequestInfo>, ContentAnalysisIdListResponseInfo>("/v3/content_analysis/id_list", body).await
}

pub async fn content_analysis_available_filters(client: &DataForSeoClient) -> Result<ContentAnalysisAvailableFiltersResponseInfo> {
    let path = "/v3/content_analysis/available_filters".to_string();
    client.get::<ContentAnalysisAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn locations(client: &DataForSeoClient) -> Result<ContentAnalysisLocationsResponseInfo> {
    let path = "/v3/content_analysis/locations".to_string();
    client.get::<ContentAnalysisLocationsResponseInfo>(
        &path
    ).await
}

pub async fn languages(client: &DataForSeoClient) -> Result<ContentAnalysisLanguagesResponseInfo> {
    let path = "/v3/content_analysis/languages".to_string();
    client.get::<ContentAnalysisLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn content_analysis_categories(client: &DataForSeoClient) -> Result<ContentAnalysisCategoriesResponseInfo> {
    let path = "/v3/content_analysis/categories".to_string();
    client.get::<ContentAnalysisCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn search_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisSearchLiveRequestInfo>) -> Result<ContentAnalysisSearchLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisSearchLiveRequestInfo>, ContentAnalysisSearchLiveResponseInfo>("/v3/content_analysis/search/live", body).await
}

pub async fn content_analysis_summary_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisSummaryLiveRequestInfo>) -> Result<ContentAnalysisSummaryLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisSummaryLiveRequestInfo>, ContentAnalysisSummaryLiveResponseInfo>("/v3/content_analysis/summary/live", body).await
}

pub async fn sentiment_analysis_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisSentimentAnalysisLiveRequestInfo>) -> Result<ContentAnalysisSentimentAnalysisLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisSentimentAnalysisLiveRequestInfo>, ContentAnalysisSentimentAnalysisLiveResponseInfo>("/v3/content_analysis/sentiment_analysis/live", body).await
}

pub async fn rating_distribution_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisRatingDistributionLiveRequestInfo>) -> Result<ContentAnalysisRatingDistributionLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisRatingDistributionLiveRequestInfo>, ContentAnalysisRatingDistributionLiveResponseInfo>("/v3/content_analysis/rating_distribution/live", body).await
}

pub async fn phrase_trends_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisPhraseTrendsLiveRequestInfo>) -> Result<ContentAnalysisPhraseTrendsLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisPhraseTrendsLiveRequestInfo>, ContentAnalysisPhraseTrendsLiveResponseInfo>("/v3/content_analysis/phrase_trends/live", body).await
}

pub async fn category_trends_live(client: &DataForSeoClient, body: &Vec<ContentAnalysisCategoryTrendsLiveRequestInfo>) -> Result<ContentAnalysisCategoryTrendsLiveResponseInfo> {
    client.post::<Vec<ContentAnalysisCategoryTrendsLiveRequestInfo>, ContentAnalysisCategoryTrendsLiveResponseInfo>("/v3/content_analysis/category_trends/live", body).await
}

pub async fn generate_live(client: &DataForSeoClient, body: &Vec<ContentGenerationGenerateLiveRequestInfo>) -> Result<ContentGenerationGenerateLiveResponseInfo> {
    client.post::<Vec<ContentGenerationGenerateLiveRequestInfo>, ContentGenerationGenerateLiveResponseInfo>("/v3/content_generation/generate/live", body).await
}

pub async fn generate_text_live(client: &DataForSeoClient, body: &Vec<ContentGenerationGenerateTextLiveRequestInfo>) -> Result<ContentGenerationGenerateTextLiveResponseInfo> {
    client.post::<Vec<ContentGenerationGenerateTextLiveRequestInfo>, ContentGenerationGenerateTextLiveResponseInfo>("/v3/content_generation/generate_text/live", body).await
}

pub async fn generate_meta_tags_live(client: &DataForSeoClient, body: &Vec<ContentGenerationGenerateMetaTagsLiveRequestInfo>) -> Result<ContentGenerationGenerateMetaTagsLiveResponseInfo> {
    client.post::<Vec<ContentGenerationGenerateMetaTagsLiveRequestInfo>, ContentGenerationGenerateMetaTagsLiveResponseInfo>("/v3/content_generation/generate_meta_tags/live", body).await
}

pub async fn generate_sub_topics_live(client: &DataForSeoClient, body: &Vec<ContentGenerationGenerateSubTopicsLiveRequestInfo>) -> Result<ContentGenerationGenerateSubTopicsLiveResponseInfo> {
    client.post::<Vec<ContentGenerationGenerateSubTopicsLiveRequestInfo>, ContentGenerationGenerateSubTopicsLiveResponseInfo>("/v3/content_generation/generate_sub_topics/live", body).await
}

pub async fn paraphrase_live(client: &DataForSeoClient, body: &Vec<ContentGenerationParaphraseLiveRequestInfo>) -> Result<ContentGenerationParaphraseLiveResponseInfo> {
    client.post::<Vec<ContentGenerationParaphraseLiveRequestInfo>, ContentGenerationParaphraseLiveResponseInfo>("/v3/content_generation/paraphrase/live", body).await
}

pub async fn check_grammar_live(client: &DataForSeoClient, body: &Vec<ContentGenerationCheckGrammarLiveRequestInfo>) -> Result<ContentGenerationCheckGrammarLiveResponseInfo> {
    client.post::<Vec<ContentGenerationCheckGrammarLiveRequestInfo>, ContentGenerationCheckGrammarLiveResponseInfo>("/v3/content_generation/check_grammar/live", body).await
}

pub async fn check_grammar_languages(client: &DataForSeoClient) -> Result<ContentGenerationCheckGrammarLanguagesResponseInfo> {
    let path = "/v3/content_generation/check_grammar/languages".to_string();
    client.get::<ContentGenerationCheckGrammarLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn grammar_rules(client: &DataForSeoClient) -> Result<ContentGenerationGrammarRulesResponseInfo> {
    let path = "/v3/content_generation/grammar_rules".to_string();
    client.get::<ContentGenerationGrammarRulesResponseInfo>(
        &path
    ).await
}

pub async fn text_summary_live(client: &DataForSeoClient, body: &Vec<ContentGenerationTextSummaryLiveRequestInfo>) -> Result<ContentGenerationTextSummaryLiveResponseInfo> {
    client.post::<Vec<ContentGenerationTextSummaryLiveRequestInfo>, ContentGenerationTextSummaryLiveResponseInfo>("/v3/content_generation/text_summary/live", body).await
}

pub async fn text_summary_languages(client: &DataForSeoClient) -> Result<ContentGenerationTextSummaryLanguagesResponseInfo> {
    let path = "/v3/content_generation/text_summary/languages".to_string();
    client.get::<ContentGenerationTextSummaryLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn merchant_id_list(client: &DataForSeoClient, body: &Vec<MerchantIdListRequestInfo>) -> Result<MerchantIdListResponseInfo> {
    client.post::<Vec<MerchantIdListRequestInfo>, MerchantIdListResponseInfo>("/v3/merchant/id_list", body).await
}

pub async fn merchant_errors(client: &DataForSeoClient, body: &Vec<MerchantErrorsRequestInfo>) -> Result<MerchantErrorsResponseInfo> {
    client.post::<Vec<MerchantErrorsRequestInfo>, MerchantErrorsResponseInfo>("/v3/merchant/errors", body).await
}

pub async fn merchant_google_languages(client: &DataForSeoClient) -> Result<MerchantGoogleLanguagesResponseInfo> {
    let path = "/v3/merchant/google/languages".to_string();
    client.get::<MerchantGoogleLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn merchant_google_locations(client: &DataForSeoClient) -> Result<MerchantGoogleLocationsResponseInfo> {
    let path = "/v3/merchant/google/locations".to_string();
    client.get::<MerchantGoogleLocationsResponseInfo>(
        &path
    ).await
}

pub async fn merchant_google_locations_country(client: &DataForSeoClient, country: &str) -> Result<MerchantGoogleLocationsCountryResponseInfo> {
    let path = format!("/v3/merchant/google/locations/{country}", country = country);
    client.get::<MerchantGoogleLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn google_products_task_post(client: &DataForSeoClient, body: &Vec<MerchantGoogleProductsTaskPostRequestInfo>) -> Result<MerchantGoogleProductsTaskPostResponseInfo> {
    client.post::<Vec<MerchantGoogleProductsTaskPostRequestInfo>, MerchantGoogleProductsTaskPostResponseInfo>("/v3/merchant/google/products/task_post", body).await
}

pub async fn google_products_tasks_ready(client: &DataForSeoClient) -> Result<MerchantGoogleProductsTasksReadyResponseInfo> {
    let path = "/v3/merchant/google/products/tasks_ready".to_string();
    client.get::<MerchantGoogleProductsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn merchant_tasks_ready(client: &DataForSeoClient) -> Result<MerchantTasksReadyResponseInfo> {
    let path = "/v3/merchant/tasks_ready".to_string();
    client.get::<MerchantTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_products_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantGoogleProductsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/google/products/task_get/advanced/{id}", id = id);
    client.get::<MerchantGoogleProductsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_products_task_get_html(client: &DataForSeoClient, id: &str) -> Result<MerchantGoogleProductsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/merchant/google/products/task_get/html/{id}", id = id);
    client.get::<MerchantGoogleProductsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_sellers_task_post(client: &DataForSeoClient, body: &Vec<MerchantGoogleSellersTaskPostRequestInfo>) -> Result<MerchantGoogleSellersTaskPostResponseInfo> {
    client.post::<Vec<MerchantGoogleSellersTaskPostRequestInfo>, MerchantGoogleSellersTaskPostResponseInfo>("/v3/merchant/google/sellers/task_post", body).await
}

pub async fn google_sellers_tasks_ready(client: &DataForSeoClient) -> Result<MerchantGoogleSellersTasksReadyResponseInfo> {
    let path = "/v3/merchant/google/sellers/tasks_ready".to_string();
    client.get::<MerchantGoogleSellersTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_sellers_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantGoogleSellersTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/google/sellers/task_get/advanced/{id}", id = id);
    client.get::<MerchantGoogleSellersTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_product_info_task_post(client: &DataForSeoClient, body: &Vec<MerchantGoogleProductInfoTaskPostRequestInfo>) -> Result<MerchantGoogleProductInfoTaskPostResponseInfo> {
    client.post::<Vec<MerchantGoogleProductInfoTaskPostRequestInfo>, MerchantGoogleProductInfoTaskPostResponseInfo>("/v3/merchant/google/product_info/task_post", body).await
}

pub async fn google_product_info_tasks_ready(client: &DataForSeoClient) -> Result<MerchantGoogleProductInfoTasksReadyResponseInfo> {
    let path = "/v3/merchant/google/product_info/tasks_ready".to_string();
    client.get::<MerchantGoogleProductInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_product_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantGoogleProductInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/google/product_info/task_get/advanced/{id}", id = id);
    client.get::<MerchantGoogleProductInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_sellers_ad_url(client: &DataForSeoClient, shop_ad_aclk: &str) -> Result<MerchantGoogleSellersAdUrlResponseInfo> {
    let path = format!("/v3/merchant/google/sellers/ad_url/{shop_ad_aclk}", shop_ad_aclk = shop_ad_aclk);
    client.get::<MerchantGoogleSellersAdUrlResponseInfo>(
        &path
    ).await
}

pub async fn amazon_locations(client: &DataForSeoClient) -> Result<MerchantAmazonLocationsResponseInfo> {
    let path = "/v3/merchant/amazon/locations".to_string();
    client.get::<MerchantAmazonLocationsResponseInfo>(
        &path
    ).await
}

pub async fn amazon_locations_country(client: &DataForSeoClient, country: &str) -> Result<MerchantAmazonLocationsCountryResponseInfo> {
    let path = format!("/v3/merchant/amazon/locations/{country}", country = country);
    client.get::<MerchantAmazonLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn amazon_languages(client: &DataForSeoClient) -> Result<MerchantAmazonLanguagesResponseInfo> {
    let path = "/v3/merchant/amazon/languages".to_string();
    client.get::<MerchantAmazonLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn amazon_products_task_post(client: &DataForSeoClient, body: &Vec<MerchantAmazonProductsTaskPostRequestInfo>) -> Result<MerchantAmazonProductsTaskPostResponseInfo> {
    client.post::<Vec<MerchantAmazonProductsTaskPostRequestInfo>, MerchantAmazonProductsTaskPostResponseInfo>("/v3/merchant/amazon/products/task_post", body).await
}

pub async fn amazon_products_tasks_ready(client: &DataForSeoClient) -> Result<MerchantAmazonProductsTasksReadyResponseInfo> {
    let path = "/v3/merchant/amazon/products/tasks_ready".to_string();
    client.get::<MerchantAmazonProductsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn amazon_products_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonProductsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/amazon/products/task_get/advanced/{id}", id = id);
    client.get::<MerchantAmazonProductsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn amazon_products_task_get_html(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonProductsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/merchant/amazon/products/task_get/html/{id}", id = id);
    client.get::<MerchantAmazonProductsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn amazon_asin_task_post(client: &DataForSeoClient, body: &Vec<MerchantAmazonAsinTaskPostRequestInfo>) -> Result<MerchantAmazonAsinTaskPostResponseInfo> {
    client.post::<Vec<MerchantAmazonAsinTaskPostRequestInfo>, MerchantAmazonAsinTaskPostResponseInfo>("/v3/merchant/amazon/asin/task_post", body).await
}

pub async fn amazon_asin_tasks_ready(client: &DataForSeoClient) -> Result<MerchantAmazonAsinTasksReadyResponseInfo> {
    let path = "/v3/merchant/amazon/asin/tasks_ready".to_string();
    client.get::<MerchantAmazonAsinTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn amazon_asin_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonAsinTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/amazon/asin/task_get/advanced/{id}", id = id);
    client.get::<MerchantAmazonAsinTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn amazon_asin_task_get_html(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonAsinTaskGetHtmlResponseInfo> {
    let path = format!("/v3/merchant/amazon/asin/task_get/html/{id}", id = id);
    client.get::<MerchantAmazonAsinTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn amazon_sellers_task_post(client: &DataForSeoClient, body: &Vec<MerchantAmazonSellersTaskPostRequestInfo>) -> Result<MerchantAmazonSellersTaskPostResponseInfo> {
    client.post::<Vec<MerchantAmazonSellersTaskPostRequestInfo>, MerchantAmazonSellersTaskPostResponseInfo>("/v3/merchant/amazon/sellers/task_post", body).await
}

pub async fn amazon_sellers_tasks_ready(client: &DataForSeoClient) -> Result<MerchantAmazonSellersTasksReadyResponseInfo> {
    let path = "/v3/merchant/amazon/sellers/tasks_ready".to_string();
    client.get::<MerchantAmazonSellersTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn amazon_sellers_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonSellersTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/amazon/sellers/task_get/advanced/{id}", id = id);
    client.get::<MerchantAmazonSellersTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn amazon_sellers_task_get_html(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonSellersTaskGetHtmlResponseInfo> {
    let path = format!("/v3/merchant/amazon/sellers/task_get/html/{id}", id = id);
    client.get::<MerchantAmazonSellersTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn amazon_reviews_task_post(client: &DataForSeoClient, body: &Vec<MerchantAmazonReviewsTaskPostRequestInfo>) -> Result<MerchantAmazonReviewsTaskPostResponseInfo> {
    client.post::<Vec<MerchantAmazonReviewsTaskPostRequestInfo>, MerchantAmazonReviewsTaskPostResponseInfo>("/v3/merchant/amazon/reviews/task_post", body).await
}

pub async fn amazon_reviews_tasks_ready(client: &DataForSeoClient) -> Result<MerchantAmazonReviewsTasksReadyResponseInfo> {
    let path = "/v3/merchant/amazon/reviews/tasks_ready".to_string();
    client.get::<MerchantAmazonReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn amazon_reviews_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonReviewsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/merchant/amazon/reviews/task_get/advanced/{id}", id = id);
    client.get::<MerchantAmazonReviewsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn amazon_reviews_task_get_html(client: &DataForSeoClient, id: &str) -> Result<MerchantAmazonReviewsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/merchant/amazon/reviews/task_get/html/{id}", id = id);
    client.get::<MerchantAmazonReviewsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn app_data_id_list(client: &DataForSeoClient, body: &Vec<AppDataIdListRequestInfo>) -> Result<AppDataIdListResponseInfo> {
    client.post::<Vec<AppDataIdListRequestInfo>, AppDataIdListResponseInfo>("/v3/app_data/id_list", body).await
}

pub async fn app_data_errors(client: &DataForSeoClient, body: &Vec<AppDataErrorsRequestInfo>) -> Result<AppDataErrorsResponseInfo> {
    client.post::<Vec<AppDataErrorsRequestInfo>, AppDataErrorsResponseInfo>("/v3/app_data/errors", body).await
}

pub async fn google_categories(client: &DataForSeoClient) -> Result<AppDataGoogleCategoriesResponseInfo> {
    let path = "/v3/app_data/google/categories".to_string();
    client.get::<AppDataGoogleCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn app_data_google_locations(client: &DataForSeoClient) -> Result<AppDataGoogleLocationsResponseInfo> {
    let path = "/v3/app_data/google/locations".to_string();
    client.get::<AppDataGoogleLocationsResponseInfo>(
        &path
    ).await
}

pub async fn app_data_google_locations_country(client: &DataForSeoClient, country: &str) -> Result<AppDataGoogleLocationsCountryResponseInfo> {
    let path = format!("/v3/app_data/google/locations/{country}", country = country);
    client.get::<AppDataGoogleLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn app_data_google_languages(client: &DataForSeoClient) -> Result<AppDataGoogleLanguagesResponseInfo> {
    let path = "/v3/app_data/google/languages".to_string();
    client.get::<AppDataGoogleLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_app_searches_task_post(client: &DataForSeoClient, body: &Vec<AppDataGoogleAppSearchesTaskPostRequestInfo>) -> Result<AppDataGoogleAppSearchesTaskPostResponseInfo> {
    client.post::<Vec<AppDataGoogleAppSearchesTaskPostRequestInfo>, AppDataGoogleAppSearchesTaskPostResponseInfo>("/v3/app_data/google/app_searches/task_post", body).await
}

pub async fn google_app_searches_tasks_ready(client: &DataForSeoClient) -> Result<AppDataGoogleAppSearchesTasksReadyResponseInfo> {
    let path = "/v3/app_data/google/app_searches/tasks_ready".to_string();
    client.get::<AppDataGoogleAppSearchesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn app_data_tasks_ready(client: &DataForSeoClient) -> Result<AppDataTasksReadyResponseInfo> {
    let path = "/v3/app_data/tasks_ready".to_string();
    client.get::<AppDataTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_app_searches_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppSearchesTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/google/app_searches/task_get/advanced/{id}", id = id);
    client.get::<AppDataGoogleAppSearchesTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_app_searches_task_get_html(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppSearchesTaskGetHtmlResponseInfo> {
    let path = format!("/v3/app_data/google/app_searches/task_get/html/{id}", id = id);
    client.get::<AppDataGoogleAppSearchesTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_app_list_task_post(client: &DataForSeoClient, body: &Vec<AppDataGoogleAppListTaskPostRequestInfo>) -> Result<AppDataGoogleAppListTaskPostResponseInfo> {
    client.post::<Vec<AppDataGoogleAppListTaskPostRequestInfo>, AppDataGoogleAppListTaskPostResponseInfo>("/v3/app_data/google/app_list/task_post", body).await
}

pub async fn google_app_list_tasks_ready(client: &DataForSeoClient) -> Result<AppDataGoogleAppListTasksReadyResponseInfo> {
    let path = "/v3/app_data/google/app_list/tasks_ready".to_string();
    client.get::<AppDataGoogleAppListTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_app_list_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppListTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/google/app_list/task_get/advanced/{id}", id = id);
    client.get::<AppDataGoogleAppListTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_app_list_task_get_html(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppListTaskGetHtmlResponseInfo> {
    let path = format!("/v3/app_data/google/app_list/task_get/html/{id}", id = id);
    client.get::<AppDataGoogleAppListTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_app_info_task_post(client: &DataForSeoClient, body: &Vec<AppDataGoogleAppInfoTaskPostRequestInfo>) -> Result<AppDataGoogleAppInfoTaskPostResponseInfo> {
    client.post::<Vec<AppDataGoogleAppInfoTaskPostRequestInfo>, AppDataGoogleAppInfoTaskPostResponseInfo>("/v3/app_data/google/app_info/task_post", body).await
}

pub async fn google_app_info_tasks_ready(client: &DataForSeoClient) -> Result<AppDataGoogleAppInfoTasksReadyResponseInfo> {
    let path = "/v3/app_data/google/app_info/tasks_ready".to_string();
    client.get::<AppDataGoogleAppInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_app_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/google/app_info/task_get/advanced/{id}", id = id);
    client.get::<AppDataGoogleAppInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_app_info_task_get_html(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppInfoTaskGetHtmlResponseInfo> {
    let path = format!("/v3/app_data/google/app_info/task_get/html/{id}", id = id);
    client.get::<AppDataGoogleAppInfoTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_app_reviews_task_post(client: &DataForSeoClient, body: &Vec<AppDataGoogleAppReviewsTaskPostRequestInfo>) -> Result<AppDataGoogleAppReviewsTaskPostResponseInfo> {
    client.post::<Vec<AppDataGoogleAppReviewsTaskPostRequestInfo>, AppDataGoogleAppReviewsTaskPostResponseInfo>("/v3/app_data/google/app_reviews/task_post", body).await
}

pub async fn google_app_reviews_tasks_ready(client: &DataForSeoClient) -> Result<AppDataGoogleAppReviewsTasksReadyResponseInfo> {
    let path = "/v3/app_data/google/app_reviews/tasks_ready".to_string();
    client.get::<AppDataGoogleAppReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_app_reviews_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppReviewsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/google/app_reviews/task_get/advanced/{id}", id = id);
    client.get::<AppDataGoogleAppReviewsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_app_reviews_task_get_html(client: &DataForSeoClient, id: &str) -> Result<AppDataGoogleAppReviewsTaskGetHtmlResponseInfo> {
    let path = format!("/v3/app_data/google/app_reviews/task_get/html/{id}", id = id);
    client.get::<AppDataGoogleAppReviewsTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_app_listings_categories(client: &DataForSeoClient) -> Result<AppDataGoogleAppListingsCategoriesResponseInfo> {
    let path = "/v3/app_data/google/app_listings/categories".to_string();
    client.get::<AppDataGoogleAppListingsCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn google_app_listings_search_live(client: &DataForSeoClient, body: &Vec<AppDataGoogleAppListingsSearchLiveRequestInfo>) -> Result<AppDataGoogleAppListingsSearchLiveResponseInfo> {
    client.post::<Vec<AppDataGoogleAppListingsSearchLiveRequestInfo>, AppDataGoogleAppListingsSearchLiveResponseInfo>("/v3/app_data/google/app_listings/search/live", body).await
}

pub async fn apple_categories(client: &DataForSeoClient) -> Result<AppDataAppleCategoriesResponseInfo> {
    let path = "/v3/app_data/apple/categories".to_string();
    client.get::<AppDataAppleCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn apple_locations(client: &DataForSeoClient) -> Result<AppDataAppleLocationsResponseInfo> {
    let path = "/v3/app_data/apple/locations".to_string();
    client.get::<AppDataAppleLocationsResponseInfo>(
        &path
    ).await
}

pub async fn apple_languages(client: &DataForSeoClient) -> Result<AppDataAppleLanguagesResponseInfo> {
    let path = "/v3/app_data/apple/languages".to_string();
    client.get::<AppDataAppleLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_searches_task_post(client: &DataForSeoClient, body: &Vec<AppDataAppleAppSearchesTaskPostRequestInfo>) -> Result<AppDataAppleAppSearchesTaskPostResponseInfo> {
    client.post::<Vec<AppDataAppleAppSearchesTaskPostRequestInfo>, AppDataAppleAppSearchesTaskPostResponseInfo>("/v3/app_data/apple/app_searches/task_post", body).await
}

pub async fn apple_app_searches_tasks_ready(client: &DataForSeoClient) -> Result<AppDataAppleAppSearchesTasksReadyResponseInfo> {
    let path = "/v3/app_data/apple/app_searches/tasks_ready".to_string();
    client.get::<AppDataAppleAppSearchesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_searches_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataAppleAppSearchesTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/apple/app_searches/task_get/advanced/{id}", id = id);
    client.get::<AppDataAppleAppSearchesTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_info_task_post(client: &DataForSeoClient, body: &Vec<AppDataAppleAppInfoTaskPostRequestInfo>) -> Result<AppDataAppleAppInfoTaskPostResponseInfo> {
    client.post::<Vec<AppDataAppleAppInfoTaskPostRequestInfo>, AppDataAppleAppInfoTaskPostResponseInfo>("/v3/app_data/apple/app_info/task_post", body).await
}

pub async fn apple_app_info_tasks_ready(client: &DataForSeoClient) -> Result<AppDataAppleAppInfoTasksReadyResponseInfo> {
    let path = "/v3/app_data/apple/app_info/tasks_ready".to_string();
    client.get::<AppDataAppleAppInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataAppleAppInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/apple/app_info/task_get/advanced/{id}", id = id);
    client.get::<AppDataAppleAppInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_list_task_post(client: &DataForSeoClient, body: &Vec<AppDataAppleAppListTaskPostRequestInfo>) -> Result<AppDataAppleAppListTaskPostResponseInfo> {
    client.post::<Vec<AppDataAppleAppListTaskPostRequestInfo>, AppDataAppleAppListTaskPostResponseInfo>("/v3/app_data/apple/app_list/task_post", body).await
}

pub async fn apple_app_list_tasks_ready(client: &DataForSeoClient) -> Result<AppDataAppleAppListTasksReadyResponseInfo> {
    let path = "/v3/app_data/apple/app_list/tasks_ready".to_string();
    client.get::<AppDataAppleAppListTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_list_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataAppleAppListTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/apple/app_list/task_get/advanced/{id}", id = id);
    client.get::<AppDataAppleAppListTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_reviews_task_post(client: &DataForSeoClient, body: &Vec<AppDataAppleAppReviewsTaskPostRequestInfo>) -> Result<AppDataAppleAppReviewsTaskPostResponseInfo> {
    client.post::<Vec<AppDataAppleAppReviewsTaskPostRequestInfo>, AppDataAppleAppReviewsTaskPostResponseInfo>("/v3/app_data/apple/app_reviews/task_post", body).await
}

pub async fn apple_app_reviews_tasks_ready(client: &DataForSeoClient) -> Result<AppDataAppleAppReviewsTasksReadyResponseInfo> {
    let path = "/v3/app_data/apple/app_reviews/tasks_ready".to_string();
    client.get::<AppDataAppleAppReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_reviews_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<AppDataAppleAppReviewsTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/app_data/apple/app_reviews/task_get/advanced/{id}", id = id);
    client.get::<AppDataAppleAppReviewsTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_listings_categories(client: &DataForSeoClient) -> Result<AppDataAppleAppListingsCategoriesResponseInfo> {
    let path = "/v3/app_data/apple/app_listings/categories".to_string();
    client.get::<AppDataAppleAppListingsCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn apple_app_listings_search_live(client: &DataForSeoClient, body: &Vec<AppDataAppleAppListingsSearchLiveRequestInfo>) -> Result<AppDataAppleAppListingsSearchLiveResponseInfo> {
    client.post::<Vec<AppDataAppleAppListingsSearchLiveRequestInfo>, AppDataAppleAppListingsSearchLiveResponseInfo>("/v3/app_data/apple/app_listings/search/live", body).await
}

pub async fn business_data_id_list(client: &DataForSeoClient, body: &Vec<BusinessDataIdListRequestInfo>) -> Result<BusinessDataIdListResponseInfo> {
    client.post::<Vec<BusinessDataIdListRequestInfo>, BusinessDataIdListResponseInfo>("/v3/business_data/id_list", body).await
}

pub async fn business_data_errors(client: &DataForSeoClient, body: &Vec<BusinessDataErrorsRequestInfo>) -> Result<BusinessDataErrorsResponseInfo> {
    client.post::<Vec<BusinessDataErrorsRequestInfo>, BusinessDataErrorsResponseInfo>("/v3/business_data/errors", body).await
}

pub async fn business_listings_locations(client: &DataForSeoClient) -> Result<BusinessDataBusinessListingsLocationsResponseInfo> {
    let path = "/v3/business_data/business_listings/locations".to_string();
    client.get::<BusinessDataBusinessListingsLocationsResponseInfo>(
        &path
    ).await
}

pub async fn business_listings_categories(client: &DataForSeoClient) -> Result<BusinessDataBusinessListingsCategoriesResponseInfo> {
    let path = "/v3/business_data/business_listings/categories".to_string();
    client.get::<BusinessDataBusinessListingsCategoriesResponseInfo>(
        &path
    ).await
}

pub async fn business_listings_available_filters(client: &DataForSeoClient) -> Result<BusinessDataBusinessListingsAvailableFiltersResponseInfo> {
    let path = "/v3/business_data/business_listings/available_filters".to_string();
    client.get::<BusinessDataBusinessListingsAvailableFiltersResponseInfo>(
        &path
    ).await
}

pub async fn business_listings_search_live(client: &DataForSeoClient, body: &Vec<BusinessDataBusinessListingsSearchLiveRequestInfo>) -> Result<BusinessDataBusinessListingsSearchLiveResponseInfo> {
    client.post::<Vec<BusinessDataBusinessListingsSearchLiveRequestInfo>, BusinessDataBusinessListingsSearchLiveResponseInfo>("/v3/business_data/business_listings/search/live", body).await
}

pub async fn business_listings_categories_aggregation_live(client: &DataForSeoClient, body: &Vec<BusinessDataBusinessListingsCategoriesAggregationLiveRequestInfo>) -> Result<BusinessDataBusinessListingsCategoriesAggregationLiveResponseInfo> {
    client.post::<Vec<BusinessDataBusinessListingsCategoriesAggregationLiveRequestInfo>, BusinessDataBusinessListingsCategoriesAggregationLiveResponseInfo>("/v3/business_data/business_listings/categories_aggregation/live", body).await
}

pub async fn business_data_google_locations(client: &DataForSeoClient) -> Result<BusinessDataGoogleLocationsResponseInfo> {
    let path = "/v3/business_data/google/locations".to_string();
    client.get::<BusinessDataGoogleLocationsResponseInfo>(
        &path
    ).await
}

pub async fn business_data_google_locations_country(client: &DataForSeoClient, country: &str) -> Result<BusinessDataGoogleLocationsCountryResponseInfo> {
    let path = format!("/v3/business_data/google/locations/{country}", country = country);
    client.get::<BusinessDataGoogleLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn business_data_google_languages(client: &DataForSeoClient) -> Result<BusinessDataGoogleLanguagesResponseInfo> {
    let path = "/v3/business_data/google/languages".to_string();
    client.get::<BusinessDataGoogleLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn google_my_business_info_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleMyBusinessInfoTaskPostRequestInfo>) -> Result<BusinessDataGoogleMyBusinessInfoTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleMyBusinessInfoTaskPostRequestInfo>, BusinessDataGoogleMyBusinessInfoTaskPostResponseInfo>("/v3/business_data/google/my_business_info/task_post", body).await
}

pub async fn google_my_business_info_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleMyBusinessInfoTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/my_business_info/tasks_ready".to_string();
    client.get::<BusinessDataGoogleMyBusinessInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn business_data_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataTasksReadyResponseInfo> {
    let path = "/v3/business_data/tasks_ready".to_string();
    client.get::<BusinessDataTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_my_business_info_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleMyBusinessInfoTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/my_business_info/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleMyBusinessInfoTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_my_business_info_live(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleMyBusinessInfoLiveRequestInfo>) -> Result<BusinessDataGoogleMyBusinessInfoLiveResponseInfo> {
    client.post::<Vec<BusinessDataGoogleMyBusinessInfoLiveRequestInfo>, BusinessDataGoogleMyBusinessInfoLiveResponseInfo>("/v3/business_data/google/my_business_info/live", body).await
}

pub async fn google_my_business_updates_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleMyBusinessUpdatesTaskPostRequestInfo>) -> Result<BusinessDataGoogleMyBusinessUpdatesTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleMyBusinessUpdatesTaskPostRequestInfo>, BusinessDataGoogleMyBusinessUpdatesTaskPostResponseInfo>("/v3/business_data/google/my_business_updates/task_post", body).await
}

pub async fn google_my_business_updates_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleMyBusinessUpdatesTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/my_business_updates/tasks_ready".to_string();
    client.get::<BusinessDataGoogleMyBusinessUpdatesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_my_business_updates_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleMyBusinessUpdatesTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/my_business_updates/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleMyBusinessUpdatesTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_searches_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleHotelSearchesTaskPostRequestInfo>) -> Result<BusinessDataGoogleHotelSearchesTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleHotelSearchesTaskPostRequestInfo>, BusinessDataGoogleHotelSearchesTaskPostResponseInfo>("/v3/business_data/google/hotel_searches/task_post", body).await
}

pub async fn google_hotel_searches_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleHotelSearchesTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/hotel_searches/tasks_ready".to_string();
    client.get::<BusinessDataGoogleHotelSearchesTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_searches_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleHotelSearchesTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/hotel_searches/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleHotelSearchesTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_searches_live(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleHotelSearchesLiveRequestInfo>) -> Result<BusinessDataGoogleHotelSearchesLiveResponseInfo> {
    client.post::<Vec<BusinessDataGoogleHotelSearchesLiveRequestInfo>, BusinessDataGoogleHotelSearchesLiveResponseInfo>("/v3/business_data/google/hotel_searches/live", body).await
}

pub async fn google_hotel_info_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleHotelInfoTaskPostRequestInfo>) -> Result<BusinessDataGoogleHotelInfoTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleHotelInfoTaskPostRequestInfo>, BusinessDataGoogleHotelInfoTaskPostResponseInfo>("/v3/business_data/google/hotel_info/task_post", body).await
}

pub async fn google_hotel_info_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleHotelInfoTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/hotel_info/tasks_ready".to_string();
    client.get::<BusinessDataGoogleHotelInfoTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_info_task_get_advanced(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleHotelInfoTaskGetAdvancedResponseInfo> {
    let path = format!("/v3/business_data/google/hotel_info/task_get/advanced/{id}", id = id);
    client.get::<BusinessDataGoogleHotelInfoTaskGetAdvancedResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_info_task_get_html(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleHotelInfoTaskGetHtmlResponseInfo> {
    let path = format!("/v3/business_data/google/hotel_info/task_get/html/{id}", id = id);
    client.get::<BusinessDataGoogleHotelInfoTaskGetHtmlResponseInfo>(
        &path
    ).await
}

pub async fn google_hotel_info_live_advanced(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleHotelInfoLiveAdvancedRequestInfo>) -> Result<BusinessDataGoogleHotelInfoLiveAdvancedResponseInfo> {
    client.post::<Vec<BusinessDataGoogleHotelInfoLiveAdvancedRequestInfo>, BusinessDataGoogleHotelInfoLiveAdvancedResponseInfo>("/v3/business_data/google/hotel_info/live/advanced", body).await
}

pub async fn google_hotel_info_live_html(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleHotelInfoLiveHtmlRequestInfo>) -> Result<BusinessDataGoogleHotelInfoLiveHtmlResponseInfo> {
    client.post::<Vec<BusinessDataGoogleHotelInfoLiveHtmlRequestInfo>, BusinessDataGoogleHotelInfoLiveHtmlResponseInfo>("/v3/business_data/google/hotel_info/live/html", body).await
}

pub async fn google_reviews_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleReviewsTaskPostRequestInfo>) -> Result<BusinessDataGoogleReviewsTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleReviewsTaskPostRequestInfo>, BusinessDataGoogleReviewsTaskPostResponseInfo>("/v3/business_data/google/reviews/task_post", body).await
}

pub async fn google_reviews_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleReviewsTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/reviews/tasks_ready".to_string();
    client.get::<BusinessDataGoogleReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_reviews_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleReviewsTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/reviews/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleReviewsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_extended_reviews_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleExtendedReviewsTaskPostRequestInfo>) -> Result<BusinessDataGoogleExtendedReviewsTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleExtendedReviewsTaskPostRequestInfo>, BusinessDataGoogleExtendedReviewsTaskPostResponseInfo>("/v3/business_data/google/extended_reviews/task_post", body).await
}

pub async fn google_extended_reviews_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleExtendedReviewsTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/extended_reviews/tasks_ready".to_string();
    client.get::<BusinessDataGoogleExtendedReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_extended_reviews_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleExtendedReviewsTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/extended_reviews/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleExtendedReviewsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_questions_and_answers_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleQuestionsAndAnswersTaskPostRequestInfo>) -> Result<BusinessDataGoogleQuestionsAndAnswersTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataGoogleQuestionsAndAnswersTaskPostRequestInfo>, BusinessDataGoogleQuestionsAndAnswersTaskPostResponseInfo>("/v3/business_data/google/questions_and_answers/task_post", body).await
}

pub async fn google_questions_and_answers_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataGoogleQuestionsAndAnswersTasksReadyResponseInfo> {
    let path = "/v3/business_data/google/questions_and_answers/tasks_ready".to_string();
    client.get::<BusinessDataGoogleQuestionsAndAnswersTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn google_questions_and_answers_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataGoogleQuestionsAndAnswersTaskGetResponseInfo> {
    let path = format!("/v3/business_data/google/questions_and_answers/task_get/{id}", id = id);
    client.get::<BusinessDataGoogleQuestionsAndAnswersTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn google_questions_and_answers_live(client: &DataForSeoClient, body: &Vec<BusinessDataGoogleQuestionsAndAnswersLiveRequestInfo>) -> Result<BusinessDataGoogleQuestionsAndAnswersLiveResponseInfo> {
    client.post::<Vec<BusinessDataGoogleQuestionsAndAnswersLiveRequestInfo>, BusinessDataGoogleQuestionsAndAnswersLiveResponseInfo>("/v3/business_data/google/questions_and_answers/live", body).await
}

pub async fn trustpilot_search_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataTrustpilotSearchTaskPostRequestInfo>) -> Result<BusinessDataTrustpilotSearchTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataTrustpilotSearchTaskPostRequestInfo>, BusinessDataTrustpilotSearchTaskPostResponseInfo>("/v3/business_data/trustpilot/search/task_post", body).await
}

pub async fn trustpilot_search_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataTrustpilotSearchTasksReadyResponseInfo> {
    let path = "/v3/business_data/trustpilot/search/tasks_ready".to_string();
    client.get::<BusinessDataTrustpilotSearchTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn trustpilot_search_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataTrustpilotSearchTaskGetResponseInfo> {
    let path = format!("/v3/business_data/trustpilot/search/task_get/{id}", id = id);
    client.get::<BusinessDataTrustpilotSearchTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn trustpilot_reviews_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataTrustpilotReviewsTaskPostRequestInfo>) -> Result<BusinessDataTrustpilotReviewsTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataTrustpilotReviewsTaskPostRequestInfo>, BusinessDataTrustpilotReviewsTaskPostResponseInfo>("/v3/business_data/trustpilot/reviews/task_post", body).await
}

pub async fn trustpilot_reviews_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataTrustpilotReviewsTasksReadyResponseInfo> {
    let path = "/v3/business_data/trustpilot/reviews/tasks_ready".to_string();
    client.get::<BusinessDataTrustpilotReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn trustpilot_reviews_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataTrustpilotReviewsTaskGetResponseInfo> {
    let path = format!("/v3/business_data/trustpilot/reviews/task_get/{id}", id = id);
    client.get::<BusinessDataTrustpilotReviewsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_locations(client: &DataForSeoClient) -> Result<BusinessDataTripadvisorLocationsResponseInfo> {
    let path = "/v3/business_data/tripadvisor/locations".to_string();
    client.get::<BusinessDataTripadvisorLocationsResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_locations_country(client: &DataForSeoClient, country: &str) -> Result<BusinessDataTripadvisorLocationsCountryResponseInfo> {
    let path = format!("/v3/business_data/tripadvisor/locations/{country}", country = country);
    client.get::<BusinessDataTripadvisorLocationsCountryResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_languages(client: &DataForSeoClient) -> Result<BusinessDataTripadvisorLanguagesResponseInfo> {
    let path = "/v3/business_data/tripadvisor/languages".to_string();
    client.get::<BusinessDataTripadvisorLanguagesResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_search_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataTripadvisorSearchTaskPostRequestInfo>) -> Result<BusinessDataTripadvisorSearchTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataTripadvisorSearchTaskPostRequestInfo>, BusinessDataTripadvisorSearchTaskPostResponseInfo>("/v3/business_data/tripadvisor/search/task_post", body).await
}

pub async fn tripadvisor_search_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataTripadvisorSearchTasksReadyResponseInfo> {
    let path = "/v3/business_data/tripadvisor/search/tasks_ready".to_string();
    client.get::<BusinessDataTripadvisorSearchTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_search_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataTripadvisorSearchTaskGetResponseInfo> {
    let path = format!("/v3/business_data/tripadvisor/search/task_get/{id}", id = id);
    client.get::<BusinessDataTripadvisorSearchTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_reviews_task_post(client: &DataForSeoClient, body: &Vec<BusinessDataTripadvisorReviewsTaskPostRequestInfo>) -> Result<BusinessDataTripadvisorReviewsTaskPostResponseInfo> {
    client.post::<Vec<BusinessDataTripadvisorReviewsTaskPostRequestInfo>, BusinessDataTripadvisorReviewsTaskPostResponseInfo>("/v3/business_data/tripadvisor/reviews/task_post", body).await
}

pub async fn tripadvisor_reviews_tasks_ready(client: &DataForSeoClient) -> Result<BusinessDataTripadvisorReviewsTasksReadyResponseInfo> {
    let path = "/v3/business_data/tripadvisor/reviews/tasks_ready".to_string();
    client.get::<BusinessDataTripadvisorReviewsTasksReadyResponseInfo>(
        &path
    ).await
}

pub async fn tripadvisor_reviews_task_get(client: &DataForSeoClient, id: &str) -> Result<BusinessDataTripadvisorReviewsTaskGetResponseInfo> {
    let path = format!("/v3/business_data/tripadvisor/reviews/task_get/{id}", id = id);
    client.get::<BusinessDataTripadvisorReviewsTaskGetResponseInfo>(
        &path
    ).await
}

pub async fn social_media_pinterest_live(client: &DataForSeoClient, body: &Vec<BusinessDataSocialMediaPinterestLiveRequestInfo>) -> Result<BusinessDataSocialMediaPinterestLiveResponseInfo> {
    client.post::<Vec<BusinessDataSocialMediaPinterestLiveRequestInfo>, BusinessDataSocialMediaPinterestLiveResponseInfo>("/v3/business_data/social_media/pinterest/live", body).await
}

pub async fn social_media_facebook_live(client: &DataForSeoClient, body: &Vec<BusinessDataSocialMediaFacebookLiveRequestInfo>) -> Result<BusinessDataSocialMediaFacebookLiveResponseInfo> {
    client.post::<Vec<BusinessDataSocialMediaFacebookLiveRequestInfo>, BusinessDataSocialMediaFacebookLiveResponseInfo>("/v3/business_data/social_media/facebook/live", body).await
}

pub async fn social_media_reddit_live(client: &DataForSeoClient, body: &Vec<BusinessDataSocialMediaRedditLiveRequestInfo>) -> Result<BusinessDataSocialMediaRedditLiveResponseInfo> {
    client.post::<Vec<BusinessDataSocialMediaRedditLiveRequestInfo>, BusinessDataSocialMediaRedditLiveResponseInfo>("/v3/business_data/social_media/reddit/live", body).await
}

pub async fn user_data(client: &DataForSeoClient) -> Result<AppendixUserDataResponseInfo> {
    let path = "/v3/appendix/user_data".to_string();
    client.get::<AppendixUserDataResponseInfo>(
        &path
    ).await
}

pub async fn errors_appendix_errors(client: &DataForSeoClient) -> Result<AppendixErrorsResponseInfo> {
    let path = "/v3/appendix/errors".to_string();
    client.get::<AppendixErrorsResponseInfo>(
        &path
    ).await
}

pub async fn webhook_resend(client: &DataForSeoClient, body: &Vec<AppendixWebhookResendRequestInfo>) -> Result<AppendixWebhookResendResponseInfo> {
    client.post::<Vec<AppendixWebhookResendRequestInfo>, AppendixWebhookResendResponseInfo>("/v3/appendix/webhook_resend", body).await
}

pub async fn appendix_status(client: &DataForSeoClient) -> Result<AppendixStatusResponseInfo> {
    let path = "/v3/appendix/status".to_string();
    client.get::<AppendixStatusResponseInfo>(
        &path
    ).await
}

