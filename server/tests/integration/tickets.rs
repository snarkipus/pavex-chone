use crate::helpers::TestApi;
use app::tickets::{Ticket, TicketForCreate};
use reqwest::{cookie::Jar, StatusCode, Url};
use std::sync::Arc;

#[tokio::test]
async fn rejects_missing_auth_token() {
    // Arrange
    let api = TestApi::spawn().await;

    // Act & Assert
    let response = api
        .api_client
        .get(format!("{}/web/tickets", &api.api_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.text().await.unwrap(), "Unauthorized Bitch");
}

#[tokio::test]
async fn rejects_bad_auth_token() {
    // Arrange
    let api = TestApi::spawn().await;

    let cookie_jar = Arc::new(Jar::default());
    let cookie = "auth-token=badtoken";
    let url = Url::parse(&api.api_address).expect("Invalid URL");
    cookie_jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to create reqwest client");

    // Act & Assert
    let response = client
        .get(format!("{}/web/tickets", &api.api_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.text().await.unwrap(), "Unauthorized Basic Bitch");
}

#[tokio::test]
async fn create_tickets() {
    // Arrange
    let api = TestApi::spawn().await;

    let cookie_jar = Arc::new(Jar::default());
    let cookie = "auth-token=user-1.exp.sign";
    let url = Url::parse(&api.api_address).expect("Invalid URL");
    cookie_jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to create reqwest client");

    let ticket_requests = [
        TicketForCreate {
            title: "Ticket 1".to_string(),
        },
        TicketForCreate {
            title: "Ticket 2".to_string(),
        },
        TicketForCreate {
            title: "Ticket 3".to_string(),
        },
    ];

    let expected_tickets = [
        Ticket {
            id: 0,
            cid: 1,
            title: "Ticket 1".to_string(),
        },
        Ticket {
            id: 1,
            cid: 1,
            title: "Ticket 2".to_string(),
        },
        Ticket {
            id: 2,
            cid: 1,
            title: "Ticket 3".to_string(),
        },
    ];

    // Act & Assert
    for (i, request) in ticket_requests.iter().enumerate() {
        let response = client
            .post(format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.json::<Ticket>().await.unwrap(),
            expected_tickets[i]
        );
    }
}

#[tokio::test]
async fn list_tickets() {
    // Arrange
    let api = TestApi::spawn().await;

    let cookie_jar = Arc::new(Jar::default());
    let cookie = "auth-token=user-1.exp.sign";
    let url = Url::parse(&api.api_address).expect("Invalid URL");
    cookie_jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to create reqwest client");

    let ticket_requests = [
        TicketForCreate {
            title: "Ticket 1".to_string(),
        },
        TicketForCreate {
            title: "Ticket 2".to_string(),
        },
        TicketForCreate {
            title: "Ticket 3".to_string(),
        },
    ];

    for request in &ticket_requests {
        client
            .post(format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");
    }

    // Act
    let response = client
        .get(format!("{}/web/tickets", &api.api_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let tickets: Vec<Ticket> = response.json().await.unwrap();
    assert_eq!(tickets.len(), 3);
}

#[tokio::test]
async fn delete_tickets() {
    // Arrange
    let api = TestApi::spawn().await;

    let cookie_jar = Arc::new(Jar::default());
    let cookie = "auth-token=user-1.exp.sign";
    let url = Url::parse(&api.api_address).expect("Invalid URL");
    cookie_jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_jar.clone())
        .build()
        .expect("Failed to create reqwest client");

    let ticket_requests = [
        TicketForCreate {
            title: "Ticket 1".to_string(),
        },
        TicketForCreate {
            title: "Ticket 2".to_string(),
        },
        TicketForCreate {
            title: "Ticket 3".to_string(),
        },
    ];

    for request in &ticket_requests {
        client
            .post(format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");
    }

    // Act
    let response = client
        .delete(format!("{}/web/tickets/{}", &api.api_address, 1))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let ticket: Ticket = response.json().await.unwrap();
    assert_eq!(ticket.title, "Ticket 2".to_string());
}
