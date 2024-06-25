use crate::helpers::TestApi;
use app::model::{Ticket, TicketForCreate};
use reqwest::StatusCode;

#[tokio::test]
async fn create_tickets() {
    // Arrange
    let api = TestApi::spawn().await;

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
            title: "Ticket 1".to_string(),
        },
        Ticket {
            id: 1,
            title: "Ticket 2".to_string(),
        },
        Ticket {
            id: 2,
            title: "Ticket 3".to_string(),
        },
    ];

    // Act & Assert
    for (i, request) in ticket_requests.iter().enumerate() {
        let response = api
            .api_client
            .post(&format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.json::<Ticket>().await.unwrap(), expected_tickets[i]);
    }
}

#[tokio::test]
async fn list_tickets() {
    // Arrange
    let api = TestApi::spawn().await;

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
        api.api_client
            .post(&format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");
    }

    // Act
    let response = api
        .api_client
        .get(&format!("{}/web/tickets", &api.api_address))
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
        api.api_client
            .post(&format!("{}/web/tickets", &api.api_address))
            .json(request)
            .send()
            .await
            .expect("Failed to execute request");
    }

    // Act
    let response = api
        .api_client
        .delete(&format!("{}/web/tickets/{}", &api.api_address, 1))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let ticket: Ticket = response.json().await.unwrap();
    assert_eq!(ticket.title, "Ticket 2".to_string());
}
